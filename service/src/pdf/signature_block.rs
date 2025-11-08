use super::{SignatureData, SignaturePosition};
use domain::Result;

/// Signature block generator for PDFs
pub struct SignatureBlockGenerator;

impl SignatureBlockGenerator {
    /// Generate text representation of signature block
    /// For MVP: Returns formatted text
    /// TODO: Generate actual PDF signature field with proper PDF library
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

    /// Generate all signature blocks
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
    /// TODO: Implement actual QR code generation
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
}
