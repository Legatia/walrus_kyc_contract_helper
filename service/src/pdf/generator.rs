use domain::{BlobId, Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};
use walrus_client::WalrusStorage;

use super::{SignatureBlockGenerator, SignatureData, VariableSubstitutor};

/// PDF Generator Service
/// Handles transformation of templates into final signed documents
pub struct PdfGenerator {
    walrus: Arc<dyn WalrusStorage>,
}

impl PdfGenerator {
    pub fn new(walrus: Arc<dyn WalrusStorage>) -> Self {
        Self { walrus }
    }

    /// Generate unsigned PDF from template + instance data
    ///
    /// This uses proper PDF manipulation with lopdf to:
    /// 1. Load the template PDF from Walrus
    /// 2. Parse the PDF structure
    /// 3. Replace {{variable}} placeholders with actual values
    /// 4. Generate a clean unsigned PDF ready for review
    pub async fn generate_from_template(
        &self,
        template_blob_id: &BlobId,
        variable_data: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        info!("Generating PDF from template: {}", template_blob_id.as_str());

        // 1. Fetch template from Walrus
        let template_pdf = self.walrus.read(template_blob_id).await?;
        debug!("Template fetched: {} bytes", template_pdf.len());

        // 2. Perform variable substitution using lopdf
        let filled_pdf = VariableSubstitutor::substitute_simple(
            &template_pdf,
            variable_data,
        )?;

        info!("PDF generated successfully: {} bytes", filled_pdf.len());
        Ok(filled_pdf)
    }

    /// Generate signed PDF with signature blocks
    ///
    /// This adds a professional signature page to the PDF with:
    /// 1. All signer information
    /// 2. Signature hashes from blockchain
    /// 3. Transaction digests for verification
    /// 4. Timestamps
    pub async fn generate_signed_pdf(
        &self,
        unsigned_blob_id: &BlobId,
        signatures: &[SignatureData],
    ) -> Result<Vec<u8>> {
        info!("Generating signed PDF from: {}", unsigned_blob_id.as_str());

        // 1. Fetch unsigned PDF
        let unsigned_pdf = self.walrus.read(unsigned_blob_id).await?;
        debug!("Unsigned PDF fetched: {} bytes", unsigned_pdf.len());

        // 2. Add signature page using lopdf
        let signed_pdf = SignatureBlockGenerator::add_signatures_to_pdf(
            &unsigned_pdf,
            signatures,
        )?;

        info!("Signed PDF generated: {} bytes", signed_pdf.len());
        Ok(signed_pdf)
    }

    /// Generate completion certificate (alternative to embedded signatures)
    ///
    /// Creates a standalone certificate document that summarizes:
    /// - Contract details
    /// - All variable data
    /// - All signatures with full verification info
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
    use chrono::Utc;
    use domain::DocumentHash;

    #[test]
    fn test_certificate_generation() {
        let mut variable_data = HashMap::new();
        variable_data.insert("customer_name".to_string(), "John Doe".to_string());
        variable_data.insert("service_plan".to_string(), "5G Premium".to_string());

        let signatures = vec![
            SignatureData {
                signer_name: "John Doe".to_string(),
                sui_address: "0x123".to_string(),
                signed_at: Utc::now(),
                signature_hash: DocumentHash::from_hex("abc123".to_string()),
                transaction_digest: "0xtx123".to_string(),
            },
        ];

        // Mock Walrus client for testing
        struct MockWalrus;

        #[async_trait::async_trait]
        impl WalrusStorage for MockWalrus {
            async fn store(&self, _data: Vec<u8>) -> Result<BlobId> {
                Ok(BlobId::new("test".to_string()))
            }

            async fn read(&self, _blob_id: &BlobId) -> Result<Vec<u8>> {
                Ok(vec![])
            }

            async fn exists(&self, _blob_id: &BlobId) -> Result<bool> {
                Ok(true)
            }

            async fn metadata(&self, _blob_id: &BlobId) -> Result<walrus_client::BlobMetadata> {
                Ok(walrus_client::BlobMetadata {
                    blob_id: "test".to_string(),
                    size: 0,
                    stored_epoch: None,
                    expiry_epoch: None,
                })
            }
        }

        let generator = PdfGenerator::new(Arc::new(MockWalrus));

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(generator.generate_completion_certificate(
            "inst_123",
            "Test Template",
            &variable_data,
            &signatures,
        ));

        assert!(result.is_ok());
        let certificate = result.unwrap();
        let text = String::from_utf8(certificate).unwrap();

        assert!(text.contains("CERTIFICATE OF COMPLETION"));
        assert!(text.contains("John Doe"));
        assert!(text.contains("5G Premium"));
    }
}
