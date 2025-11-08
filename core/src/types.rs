use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Unique identifier for a blob stored in Walrus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlobId(pub String);

impl BlobId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Unique identifier for a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// Unique identifier for a contract
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContractId(pub Uuid);

impl ContractId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for ContractId {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash of document content (SHA-256)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentHash(pub String);

impl DocumentHash {
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Self(hex::encode(result))
    }

    pub fn from_hex(hex: String) -> Self {
        Self(hex)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// KYC document types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    Passport,
    DriverLicense,
    NationalId,
    ProofOfAddress,
    Other(String),
}

/// KYC verification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Approved,
    Rejected,
    RequiresReview,
}

/// KYC Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycDocument {
    pub id: Uuid,
    pub user_id: UserId,
    pub document_type: DocumentType,
    pub blob_id: BlobId,
    pub document_hash: DocumentHash,
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
    pub verification_status: VerificationStatus,
    pub verified_at: Option<DateTime<Utc>>,
    pub sui_object_id: Option<String>, // Reference to on-chain record
}

/// Contract metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: ContractId,
    pub title: String,
    pub description: Option<String>,
    pub blob_id: BlobId,
    pub document_hash: DocumentHash,
    pub created_at: DateTime<Utc>,
    pub created_by: UserId,
    pub signers: Vec<Signer>,
    pub sui_object_id: Option<String>, // Reference to on-chain record
    pub status: ContractStatus,
}

/// Contract status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    Draft,
    PendingSignatures,
    FullySigned,
    Expired,
    Cancelled,
}

/// Signer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signer {
    pub user_id: UserId,
    pub sui_address: String,
    pub signed_at: Option<DateTime<Utc>>,
    pub signature: Option<String>,
    pub signature_hash: Option<DocumentHash>,
}

/// Compliance event for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvent {
    pub id: Uuid,
    pub event_type: ComplianceEventType,
    pub user_id: Option<UserId>,
    pub resource_id: String, // Could be KYC doc ID, contract ID, etc.
    pub resource_type: ResourceType,
    pub action: String,
    pub metadata: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub sui_object_id: Option<String>, // On-chain event record
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceEventType {
    KycUpload,
    KycVerification,
    ContractCreated,
    ContractSigned,
    DocumentAccessed,
    DocumentDeleted,
    GdprRequest,
    AuditLog,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    KycDocument,
    Contract,
    User,
    System,
}

/// Configuration for Walrus storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalrusConfig {
    pub publisher_url: String,
    pub aggregator_url: String,
    pub epochs: u32, // Storage duration in epochs
}

/// Configuration for Sui blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiConfig {
    pub rpc_url: String,
    pub package_id: String,
    pub private_key: String,
}
