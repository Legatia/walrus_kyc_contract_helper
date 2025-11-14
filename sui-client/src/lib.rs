use async_trait::async_trait;
use domain::{AuditEvent, Contract, DocumentHash, Result};

pub mod client;

pub use client::SuiClient;

/// Trait for Sui blockchain operations
#[async_trait]
pub trait SuiBlockchain: Send + Sync {
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

    /// Log an audit event on-chain
    async fn log_audit_event(
        &self,
        event: &AuditEvent,
    ) -> Result<String>; // Returns Sui object ID

    /// Get contract by object ID
    async fn get_contract(&self, object_id: &str) -> Result<Contract>;
}
