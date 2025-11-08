use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Walrus storage error: {0}")]
    WalrusError(String),

    #[error("Sui blockchain error: {0}")]
    SuiError(String),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Compliance violation: {0}")]
    ComplianceViolation(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
