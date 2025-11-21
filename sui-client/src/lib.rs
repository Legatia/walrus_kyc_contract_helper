use async_trait::async_trait;
use domain::{AuditEvent, Contract, ContractInstance, DocumentHash, Result};

pub mod client;

pub use client::SuiClient;

/// Payment validation result
#[derive(Debug, Clone)]
pub struct PaymentValidation {
    pub valid: bool,
    pub amount: u64,
    pub sender: String,
    pub recipient: String,
    pub transaction_digest: String,
}

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

    /// Validate a payment transaction
    /// Verifies that the transaction exists, is confirmed, and transferred the correct amount
    async fn validate_payment(
        &self,
        transaction_digest: &str,
        expected_recipient: &str,
        expected_amount: u64,
    ) -> Result<PaymentValidation>;

    /// Get contract instance by ID
    async fn get_instance(&self, instance_id: &str) -> Result<ContractInstance>;
}
