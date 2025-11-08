use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{BlobId, DocumentHash, UserId};

/// Contract template with variable placeholders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTemplate {
    pub id: TemplateId,
    pub name: String,
    pub description: String,
    pub template_blob_id: BlobId,        // PDF with {{variable}} placeholders
    pub creator: UserId,
    pub creator_address: String,          // Sui address
    pub category: TemplateCategory,
    pub variables: Vec<String>,           // ["customer_name", "service_plan", ...]
    pub price_per_use: u64,              // In MIST (1 SUI = 1_000_000_000 MIST)
    pub royalty_percentage: u8,           // 0-100
    pub usage_count: u64,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub version: u64,
    pub sui_object_id: Option<String>,
}

/// Unique identifier for templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TemplateId(pub Uuid);

impl TemplateId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TemplateId {
    fn default() -> Self {
        Self::new()
    }
}

/// Template categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCategory {
    Mvno,              // Mobile Virtual Network Operator
    Saas,              // Software as a Service
    Nda,               // Non-Disclosure Agreement
    Employment,        // Employment contracts
    Service,           // Service agreements
    Rental,            // Rental agreements
    Other(String),
}

/// Contract instance created from a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInstance {
    pub id: InstanceId,
    pub template_id: TemplateId,
    pub instance_blob_id: BlobId,        // Generated PDF with filled variables
    pub variable_data: HashMap<String, String>, // Actual values
    pub created_by: UserId,
    pub required_signers: Vec<SignerInfo>,
    pub status: InstanceStatus,
    pub payment_tx: Option<String>,      // Sui transaction digest
    pub created_at: DateTime<Utc>,
    pub sui_object_id: Option<String>,
}

/// Unique identifier for contract instances
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InstanceId(pub Uuid);

impl InstanceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for InstanceId {
    fn default() -> Self {
        Self::new()
    }
}

/// Instance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstanceStatus {
    Draft,
    PendingSignatures,
    FullySigned,
    Expired,
}

/// Signer information for instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerInfo {
    pub user_id: UserId,
    pub sui_address: String,
    pub role: String,                    // "customer", "operator", "witness"
    pub signed_at: Option<DateTime<Utc>>,
    pub signature_hash: Option<DocumentHash>,
}

/// Template marketplace listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMarketplaceListing {
    pub template: ContractTemplate,
    pub preview_url: String,             // Preview the template
    pub rating: f32,                     // User ratings
    pub review_count: u32,
}

/// Payment information for template usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePayment {
    pub template_id: TemplateId,
    pub payer: UserId,
    pub amount: u64,
    pub transaction_digest: String,
    pub paid_at: DateTime<Utc>,
}

/// Variable substitution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSubstitution {
    pub variable_name: String,
    pub value: String,
    pub value_type: VariableType,        // For validation
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    Text,
    Number,
    Date,
    Currency,
    Email,
    Address,
}
