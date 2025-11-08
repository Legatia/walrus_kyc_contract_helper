pub mod generator;
pub mod substitution;
pub mod signature_block;

pub use generator::PdfGenerator;
pub use substitution::VariableSubstitutor;
pub use signature_block::SignatureBlockGenerator;

use chrono::{DateTime, Utc};
use domain::DocumentHash;
use serde::{Deserialize, Serialize};

/// Position for signature block on PDF
#[derive(Debug, Clone)]
pub struct SignaturePosition {
    pub x: f32,
    pub y: f32,
    pub qr_x: f32,
    pub qr_y: f32,
}

impl Default for SignaturePosition {
    fn default() -> Self {
        Self {
            x: 50.0,
            y: 100.0,
            qr_x: 450.0,
            qr_y: 100.0,
        }
    }
}

/// Signature data for PDF generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureData {
    pub signer_name: String,
    pub sui_address: String,
    pub signed_at: DateTime<Utc>,
    pub signature_hash: DocumentHash,
    pub transaction_digest: String,
}

/// PDF generation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationStrategy {
    Native,      // Use Rust PDF libraries
    External,    // Use external service (e.g., PDFMonkey)
    Auto,        // Choose based on complexity
}
