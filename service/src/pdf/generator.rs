use domain::{BlobId, Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};
use walrus_client::WalrusStorage;

use super::{SignatureData, VariableSubstitutor};

/// PDF Generator Service
pub struct PdfGenerator {
    walrus: Arc<dyn WalrusStorage>,
}

impl PdfGenerator {
    pub fn new(walrus: Arc<dyn WalrusStorage>) -> Self {
        Self { walrus }
    }

    /// Generate unsigned PDF from template + instance data
    pub async fn generate_from_template(
        &self,
        template_blob_id: &BlobId,
        variable_data: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        info!("Generating PDF from template: {}", template_blob_id.as_str());

        // 1. Fetch template from Walrus
        let template_pdf = self.walrus.read(template_blob_id).await?;
        debug!("Template fetched: {} bytes", template_pdf.len());

        // 2. Perform variable substitution
        // For MVP: Simple text-based substitution
        // TODO: Use PDF library for proper manipulation
        let filled_pdf = VariableSubstitutor::substitute_simple(
            &template_pdf,
            variable_data,
        )?;

        info!("PDF generated successfully: {} bytes", filled_pdf.len());
        Ok(filled_pdf)
    }

    /// Generate signed PDF with signature blocks
    pub async fn generate_signed_pdf(
        &self,
        unsigned_blob_id: &BlobId,
        signatures: &[SignatureData],
    ) -> Result<Vec<u8>> {
        info!("Generating signed PDF from: {}", unsigned_blob_id.as_str());

        // 1. Fetch unsigned PDF
        let unsigned_pdf = self.walrus.read(unsigned_blob_id).await?;

        // 2. Add signature information
        // For MVP: Append signature page
        // TODO: Use PDF library to embed signatures properly
        let signed_pdf = Self::append_signature_page(&unsigned_pdf, signatures)?;

        info!("Signed PDF generated: {} bytes", signed_pdf.len());
        Ok(signed_pdf)
    }

    /// Simple implementation: Append signature page as text
    /// TODO: Replace with proper PDF manipulation
    fn append_signature_page(
        unsigned_pdf: &[u8],
        signatures: &[SignatureData],
    ) -> Result<Vec<u8>> {
        // For MVP, we'll create a completion certificate instead
        // This is simpler than PDF manipulation

        let mut result = unsigned_pdf.to_vec();

        // Add marker for signature section
        let signature_section = format!(
            "\n\n=== DIGITAL SIGNATURES ===\n\n{}",
            signatures
                .iter()
                .enumerate()
                .map(|(i, sig)| {
                    format!(
                        "Signature {}\n\
                         Signer: {}\n\
                         Sui Address: {}\n\
                         Signed At: {}\n\
                         Signature Hash: {}\n\
                         Transaction: {}\n\
                         \n",
                        i + 1,
                        sig.signer_name,
                        sig.sui_address,
                        sig.signed_at.format("%Y-%m-%d %H:%M:%S UTC"),
                        sig.signature_hash.as_str(),
                        sig.transaction_digest,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        );

        result.extend_from_slice(signature_section.as_bytes());

        Ok(result)
    }

    /// Generate completion certificate (alternative to embedded signatures)
    pub async fn generate_completion_certificate(
        &self,
        instance_id: &str,
        template_name: &str,
        variable_data: &HashMap<String, String>,
        signatures: &[SignatureData],
    ) -> Result<Vec<u8>> {
        info!("Generating completion certificate for instance: {}", instance_id);

        let certificate = format!(
            "╔══════════════════════════════════════════════════════════╗\n\
             ║          CERTIFICATE OF COMPLETION                      ║\n\
             ╚══════════════════════════════════════════════════════════╝\n\
             \n\
             Contract Instance: {}\n\
             Template: {}\n\
             Generated: {}\n\
             \n\
             ──────────────────────────────────────────────────────────\n\
             CONTRACT DETAILS\n\
             ──────────────────────────────────────────────────────────\n\
             \n\
             {}\n\
             \n\
             ──────────────────────────────────────────────────────────\n\
             SIGNATURES ({})\n\
             ──────────────────────────────────────────────────────────\n\
             \n\
             {}\n\
             \n\
             ──────────────────────────────────────────────────────────\n\
             VERIFICATION\n\
             ──────────────────────────────────────────────────────────\n\
             \n\
             This certificate confirms that all parties have digitally\n\
             signed the contract on the Sui blockchain.\n\
             \n\
             Verify on-chain:\n\
             Instance ID: {}\n\
             \n\
             All signatures are cryptographically verifiable and\n\
             immutably recorded on the blockchain.\n\
             \n\
             ══════════════════════════════════════════════════════════\n",
            instance_id,
            template_name,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            variable_data
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n"),
            signatures.len(),
            signatures
                .iter()
                .enumerate()
                .map(|(i, sig)| {
                    format!(
                        "{}. {}\n\
                         Sui Address: {}\n\
                         Signed: {}\n\
                         Signature: {}\n\
                         Tx: {}",
                        i + 1,
                        sig.signer_name,
                        sig.sui_address,
                        sig.signed_at.format("%Y-%m-%d %H:%M:%S UTC"),
                        &sig.signature_hash.as_str()[..16],
                        &sig.transaction_digest[..16],
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n"),
            instance_id,
        );

        Ok(certificate.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_signature_page() {
        let pdf = b"Mock PDF content";
        let sigs = vec![SignatureData {
            signer_name: "Test User".to_string(),
            sui_address: "0x123".to_string(),
            signed_at: chrono::Utc::now(),
            signature_hash: domain::DocumentHash::from_hex("abc123".to_string()),
            transaction_digest: "0xtx123".to_string(),
        }];

        let result = PdfGenerator::append_signature_page(pdf, &sigs);
        assert!(result.is_ok());

        let signed = result.unwrap();
        assert!(signed.len() > pdf.len());
    }
}
