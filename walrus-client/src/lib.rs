use async_trait::async_trait;
use domain::{BlobId, Error, Result};
use serde::{Deserialize, Serialize};

pub mod client;

pub use client::WalrusClient;

/// Trait for Walrus storage operations
#[async_trait]
pub trait WalrusStorage: Send + Sync {
    /// Store a blob and return its ID
    async fn store(&self, data: Vec<u8>) -> Result<BlobId>;

    /// Retrieve a blob by ID
    async fn read(&self, blob_id: &BlobId) -> Result<Vec<u8>>;

    /// Check if a blob exists
    async fn exists(&self, blob_id: &BlobId) -> Result<bool>;

    /// Get blob metadata
    async fn metadata(&self, blob_id: &BlobId) -> Result<BlobMetadata>;
}

/// Metadata about a stored blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobMetadata {
    pub blob_id: String,
    pub size: u64,
    pub stored_epoch: Option<u64>,
    pub expiry_epoch: Option<u64>,
}

/// Response from Walrus storage
#[derive(Debug, Deserialize)]
pub struct StoreResponse {
    #[serde(rename = "newlyCreated")]
    pub newly_created: Option<NewlyCreatedBlob>,
    #[serde(rename = "alreadyCertified")]
    pub already_certified: Option<AlreadyCertifiedBlob>,
}

#[derive(Debug, Deserialize)]
pub struct NewlyCreatedBlob {
    #[serde(rename = "blobObject")]
    pub blob_object: BlobObject,
    pub encoded: EncodedBlob,
}

#[derive(Debug, Deserialize)]
pub struct AlreadyCertifiedBlob {
    #[serde(rename = "blobId")]
    pub blob_id: String,
    #[serde(rename = "eventOrObject")]
    pub event_or_object: EventOrObject,
}

#[derive(Debug, Deserialize)]
pub struct BlobObject {
    pub id: String,
    #[serde(rename = "storedEpoch")]
    pub stored_epoch: u64,
    #[serde(rename = "blobId")]
    pub blob_id: String,
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct EncodedBlob {
    pub id: String,
    #[serde(rename = "blobId")]
    pub blob_id: String,
    pub size: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventOrObject {
    Event,
    Object,
}

impl StoreResponse {
    pub fn blob_id(&self) -> Result<String> {
        if let Some(newly_created) = &self.newly_created {
            Ok(newly_created.blob_object.blob_id.clone())
        } else if let Some(already_certified) = &self.already_certified {
            Ok(already_certified.blob_id.clone())
        } else {
            Err(Error::WalrusError(
                "Invalid store response: no blob ID found".to_string(),
            ))
        }
    }
}
