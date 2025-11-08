use async_trait::async_trait;
use domain::{BlobId, ComplianceEvent, Contract, DocumentHash, KycDocument, Result, UserId};

pub mod client;

pub use client::SuiClient;

/// Trait for Sui blockchain operations
#[async_trait]
pub trait SuiBlockchain: Send + Sync {
    /// Register a KYC document on-chain
    async fn register_kyc_document(
        &self,
        user_id: &UserId,
        blob_id: &BlobId,
        document_hash: &DocumentHash,
    ) -> Result<String>; // Returns Sui object ID

    /// Update KYC verification status
    async fn update_kyc_status(
        &self,
        object_id: &str,
        status: &str,
    ) -> Result<()>;

    /// Register a contract on-chain
    async fn register_contract(
        &self,
        contract: &Contract,
    ) -> Result<String>; // Returns Sui object ID

    /// Record a contract signature on-chain
    async fn record_signature(
        &self,
        contract_object_id: &str,
        signer_address: &str,
        signature_hash: &DocumentHash,
    ) -> Result<()>;

    /// Verify a signature on-chain
    async fn verify_signature(
        &self,
        contract_object_id: &str,
        signer_address: &str,
    ) -> Result<bool>;

    /// Log a compliance event on-chain
    async fn log_compliance_event(
        &self,
        event: &ComplianceEvent,
    ) -> Result<String>; // Returns Sui object ID

    /// Get KYC document by object ID
    async fn get_kyc_document(&self, object_id: &str) -> Result<KycDocument>;

    /// Get contract by object ID
    async fn get_contract(&self, object_id: &str) -> Result<Contract>;

    /// Query KYC documents for a user
    async fn query_user_kyc_documents(&self, user_id: &UserId) -> Result<Vec<String>>; // Returns object IDs
}
