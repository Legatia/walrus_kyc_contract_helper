use async_trait::async_trait;
use core::{BlobId, Error, Result};
use reqwest::Client;
use tracing::{debug, info};

use crate::{BlobMetadata, StoreResponse, WalrusStorage};

/// Walrus HTTP client implementation
#[derive(Clone)]
pub struct WalrusClient {
    publisher_url: String,
    aggregator_url: String,
    epochs: u32,
    http_client: Client,
}

impl WalrusClient {
    pub fn new(publisher_url: String, aggregator_url: String, epochs: u32) -> Self {
        Self {
            publisher_url,
            aggregator_url,
            epochs,
            http_client: Client::new(),
        }
    }

    /// Build the store URL with epochs parameter
    fn store_url(&self) -> String {
        format!("{}/v1/store?epochs={}", self.publisher_url, self.epochs)
    }

    /// Build the read URL for a blob ID
    fn read_url(&self, blob_id: &str) -> String {
        format!("{}/v1/{}", self.aggregator_url, blob_id)
    }
}

#[async_trait]
impl WalrusStorage for WalrusClient {
    async fn store(&self, data: Vec<u8>) -> Result<BlobId> {
        debug!("Storing blob of size {} bytes", data.len());

        let response = self
            .http_client
            .put(&self.store_url())
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await
            .map_err(|e| Error::WalrusError(format!("Failed to store blob: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::WalrusError(format!(
                "Store request failed with status {}: {}",
                status, error_text
            )));
        }

        let store_response: StoreResponse = response
            .json()
            .await
            .map_err(|e| Error::WalrusError(format!("Failed to parse store response: {}", e)))?;

        let blob_id = store_response.blob_id()?;
        info!("Successfully stored blob: {}", blob_id);

        Ok(BlobId::new(blob_id))
    }

    async fn read(&self, blob_id: &BlobId) -> Result<Vec<u8>> {
        debug!("Reading blob: {}", blob_id.as_str());

        let response = self
            .http_client
            .get(&self.read_url(blob_id.as_str()))
            .send()
            .await
            .map_err(|e| Error::WalrusError(format!("Failed to read blob: {}", e)))?;

        if !response.status().is_success() {
            if response.status().as_u16() == 404 {
                return Err(Error::DocumentNotFound(blob_id.as_str().to_string()));
            }
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::WalrusError(format!(
                "Read request failed with status {}: {}",
                status, error_text
            )));
        }

        let data = response
            .bytes()
            .await
            .map_err(|e| Error::WalrusError(format!("Failed to read blob data: {}", e)))?
            .to_vec();

        info!("Successfully read blob: {} ({} bytes)", blob_id.as_str(), data.len());

        Ok(data)
    }

    async fn exists(&self, blob_id: &BlobId) -> Result<bool> {
        debug!("Checking existence of blob: {}", blob_id.as_str());

        let response = self
            .http_client
            .head(&self.read_url(blob_id.as_str()))
            .send()
            .await
            .map_err(|e| Error::WalrusError(format!("Failed to check blob existence: {}", e)))?;

        Ok(response.status().is_success())
    }

    async fn metadata(&self, blob_id: &BlobId) -> Result<BlobMetadata> {
        debug!("Getting metadata for blob: {}", blob_id.as_str());

        let response = self
            .http_client
            .head(&self.read_url(blob_id.as_str()))
            .send()
            .await
            .map_err(|e| Error::WalrusError(format!("Failed to get blob metadata: {}", e)))?;

        if !response.status().is_success() {
            if response.status().as_u16() == 404 {
                return Err(Error::DocumentNotFound(blob_id.as_str().to_string()));
            }
            return Err(Error::WalrusError(format!(
                "Metadata request failed with status {}",
                response.status()
            )));
        }

        // Extract size from Content-Length header
        let size = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(BlobMetadata {
            blob_id: blob_id.as_str().to_string(),
            size,
            stored_epoch: None,  // Would need additional API call to get this
            expiry_epoch: None,  // Would need additional API call to get this
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = WalrusClient::new(
            "https://publisher.example.com".to_string(),
            "https://aggregator.example.com".to_string(),
            5,
        );
        assert_eq!(
            client.store_url(),
            "https://publisher.example.com/v1/store?epochs=5"
        );
    }
}
