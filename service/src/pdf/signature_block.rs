use super::{SignatureData, SignaturePosition};
use domain::Result;
use lopdf::{Dictionary, Document, Object, ObjectId, Stream};
use tracing::{debug, info};

/// Signature block generator for PDFs
/// Uses lopdf for PDF manipulation and printpdf for rendering
pub struct SignatureBlockGenerator;

impl SignatureBlockGenerator {
    /// Add signature annotations to PDF document
    pub fn add_signatures_to_pdf(
        pdf_bytes: &[u8],
        signatures: &[SignatureData],
    ) -> Result<Vec<u8>> {
        info!("Adding {} signatures to PDF", signatures.len());

        // Load existing PDF
        let mut doc =
            Document::load_mem(pdf_bytes).map_err(|e| domain::Error::InvalidDocument(format!("Failed to load PDF: {}", e)))?;

        debug!("PDF loaded, {} pages", doc.get_pages().len());

        // Add a new page for signatures
        let signature_page_id = Self::add_signature_page(&mut doc, signatures)?;

        debug!("Signature page added with ID: {:?}", signature_page_id);

        // Save modified PDF
        let mut output = Vec::new();
        doc.save_to(&mut output)
            .map_err(|e| domain::Error::Internal(format!("Failed to save PDF: {}", e)))?;

        info!("Signatures added successfully, output size: {} bytes", output.len());

        Ok(output)
    }

    /// Add a new page with signature information
    fn add_signature_page(
        doc: &mut Document,
        signatures: &[SignatureData],
    ) -> Result<ObjectId> {
        // Get the last page to determine page dimensions
        let pages = doc.get_pages();
        let (_last_page_num, last_page_id) = pages.iter().last().unwrap();

        let page_obj = doc.get_object(*last_page_id).unwrap();
        let page_dict = page_obj.as_dict().unwrap();

        // Get MediaBox for page dimensions
        let media_box = if let Ok(mb) = page_dict.get(b"MediaBox") {
            mb.clone()
        } else {
            // Default US Letter size
            Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                Object::Integer(612), // 8.5 inches * 72 DPI
                Object::Integer(792), // 11 inches * 72 DPI
            ])
        };

        // Create signature page content stream
        let content = Self::generate_signature_page_content(signatures)?;

        // Create content stream object
        let content_stream = Stream::new(Dictionary::new(), content.into_bytes());
        let content_id = doc.add_object(content_stream);

        // Create new page dictionary
        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("MediaBox", media_box);
        page_dict.set("Contents", Object::Reference(content_id));

        // Get Pages object
        let catalog = doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
        let catalog_obj = doc.get_object(catalog).unwrap().as_dict().unwrap();
        let pages_ref = catalog_obj.get(b"Pages").unwrap().as_reference().unwrap();

        page_dict.set("Parent", Object::Reference(pages_ref));

        // Add page to document
        let page_id = doc.add_object(page_dict);

        // Add page to Pages array
        let pages_obj = doc.get_object_mut(pages_ref).unwrap();
        let pages_dict = pages_obj.as_dict_mut().unwrap();

        if let Ok(kids) = pages_dict.get_mut(b"Kids") {
            if let Object::Array(ref mut kids_array) = kids {
                kids_array.push(Object::Reference(page_id));
            }
        }

        // Update page count
        if let Ok(count) = pages_dict.get_mut(b"Count") {
            if let Object::Integer(n) = count {
                *n += 1;
            }
        }

        Ok(page_id)
    }

    /// Generate PDF content stream for signature page
    fn generate_signature_page_content(signatures: &[SignatureData]) -> Result<String> {
        let mut content = String::new();

        // PDF content stream commands
        content.push_str("BT\n"); // Begin text

        // Set font (Helvetica, 12pt)
        content.push_str("/F1 12 Tf\n");

        // Title
        content.push_str("50 750 Td\n"); // Position
        content.push_str("(DIGITAL SIGNATURES) Tj\n");

        content.push_str("0 -20 Td\n");
        content.push_str("(==========================================) Tj\n");

        let mut y_offset = -30;

        for (i, sig) in signatures.iter().enumerate() {
            // Signature number
            content.push_str(&format!("0 {} Td\n", y_offset));
            content.push_str(&format!("(Signature {} of {}) Tj\n", i + 1, signatures.len()));

            // Signer name
            content.push_str("0 -15 Td\n");
            content.push_str(&format!("(Signed by: {}) Tj\n", Self::escape_pdf_string(&sig.signer_name)));

            // Address
            content.push_str("0 -12 Td\n");
            content.push_str(&format!("(Sui Address: {}) Tj\n", Self::escape_pdf_string(&sig.sui_address)));

            // Date
            content.push_str("0 -12 Td\n");
            content.push_str(&format!(
                "(Date: {}) Tj\n",
                Self::escape_pdf_string(&sig.signed_at.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            ));

            // Signature hash (truncated for display)
            content.push_str("0 -12 Td\n");
            let hash_display = &sig.signature_hash.as_str()[..std::cmp::min(32, sig.signature_hash.as_str().len())];
            content.push_str(&format!("(Signature: {}...) Tj\n", Self::escape_pdf_string(hash_display)));

            // Transaction digest
            content.push_str("0 -12 Td\n");
            let tx_display = &sig.transaction_digest[..std::cmp::min(32, sig.transaction_digest.len())];
            content.push_str(&format!("(TX: {}...) Tj\n", Self::escape_pdf_string(tx_display)));

            // Verification link
            content.push_str("/F1 10 Tf\n"); // Smaller font for URL
            content.push_str("0 -12 Td\n");
            content.push_str(&format!(
                "(Verify: https://suiscan.xyz/mainnet/tx/{}) Tj\n",
                Self::escape_pdf_string(&sig.transaction_digest)
            ));

            content.push_str("/F1 12 Tf\n"); // Back to normal font

            // Separator
            content.push_str("0 -20 Td\n");
            content.push_str("(------------------------------------------) Tj\n");

            y_offset = -30;
        }

        // Footer
        content.push_str("0 -40 Td\n");
        content.push_str("(All signatures are cryptographically verifiable) Tj\n");
        content.push_str("0 -15 Td\n");
        content.push_str("(on the Sui blockchain.) Tj\n");

        content.push_str("ET\n"); // End text

        Ok(content)
    }

    /// Escape special characters in PDF strings
    fn escape_pdf_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }

    /// Generate text representation of signature block (for compatibility)
    pub fn generate_text_block(signature: &SignatureData) -> String {
        format!(
            "╔════════════════════════════════════════════════════════╗\n\
             ║ DIGITAL SIGNATURE                                      ║\n\
             ╚════════════════════════════════════════════════════════╝\n\
             \n\
             Signed by: {}\n\
             Sui Address: {}\n\
             Date & Time: {}\n\
             \n\
             Signature Hash:\n\
             {}\n\
             \n\
             Transaction Digest:\n\
             {}\n\
             \n\
             ────────────────────────────────────────────────────────\n\
             Verify on Sui blockchain:\n\
             https://suiscan.xyz/mainnet/tx/{}\n\
             ────────────────────────────────────────────────────────\n",
            signature.signer_name,
            signature.sui_address,
            signature.signed_at.format("%Y-%m-%d %H:%M:%S UTC"),
            signature.signature_hash.as_str(),
            signature.transaction_digest,
            signature.transaction_digest,
        )
    }

    /// Generate all signature blocks as text
    pub fn generate_all_blocks(signatures: &[SignatureData]) -> String {
        let mut blocks = String::new();

        blocks.push_str("\n\n");
        blocks.push_str("═══════════════════════════════════════════════════════════\n");
        blocks.push_str("                   DIGITAL SIGNATURES                      \n");
        blocks.push_str("═══════════════════════════════════════════════════════════\n");
        blocks.push_str("\n");

        for (index, sig) in signatures.iter().enumerate() {
            blocks.push_str(&format!("Signature {} of {}\n\n", index + 1, signatures.len()));
            blocks.push_str(&Self::generate_text_block(sig));
            blocks.push_str("\n\n");
        }

        blocks.push_str("═══════════════════════════════════════════════════════════\n");
        blocks.push_str("All signatures are cryptographically verifiable on-chain.\n");
        blocks.push_str("═══════════════════════════════════════════════════════════\n");

        blocks
    }

    /// Generate QR code data URL (for verification link)
    pub fn generate_qr_code_url(transaction_digest: &str) -> String {
        format!("https://suiscan.xyz/mainnet/tx/{}", transaction_digest)
    }

    /// Calculate signature block position on page
    pub fn calculate_position(
        page_height: f32,
        signature_index: usize,
        total_signatures: usize,
    ) -> SignaturePosition {
        // Distribute signatures evenly on page
        let spacing = page_height / (total_signatures as f32 + 1.0);
        let y_position = spacing * (signature_index as f32 + 1.0);

        SignaturePosition {
            x: 50.0,
            y: y_position,
            qr_x: 450.0,
            qr_y: y_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use domain::DocumentHash;

    #[test]
    fn test_generate_text_block() {
        let sig = SignatureData {
            signer_name: "John Doe".to_string(),
            sui_address: "0x123abc".to_string(),
            signed_at: Utc::now(),
            signature_hash: DocumentHash::from_hex("hash123".to_string()),
            transaction_digest: "0xtx123".to_string(),
        };

        let block = SignatureBlockGenerator::generate_text_block(&sig);

        assert!(block.contains("John Doe"));
        assert!(block.contains("0x123abc"));
        assert!(block.contains("0xtx123"));
    }

    #[test]
    fn test_generate_all_blocks() {
        let sigs = vec![
            SignatureData {
                signer_name: "Alice".to_string(),
                sui_address: "0xaaa".to_string(),
                signed_at: Utc::now(),
                signature_hash: DocumentHash::from_hex("hash1".to_string()),
                transaction_digest: "0xtx1".to_string(),
            },
            SignatureData {
                signer_name: "Bob".to_string(),
                sui_address: "0xbbb".to_string(),
                signed_at: Utc::now(),
                signature_hash: DocumentHash::from_hex("hash2".to_string()),
                transaction_digest: "0xtx2".to_string(),
            },
        ];

        let blocks = SignatureBlockGenerator::generate_all_blocks(&sigs);

        assert!(blocks.contains("Signature 1 of 2"));
        assert!(blocks.contains("Signature 2 of 2"));
        assert!(blocks.contains("Alice"));
        assert!(blocks.contains("Bob"));
    }

    #[test]
    fn test_escape_pdf_string() {
        let input = "Test (with) special\\chars";
        let escaped = SignatureBlockGenerator::escape_pdf_string(input);
        assert_eq!(escaped, "Test \\(with\\) special\\\\chars");
    }
}
