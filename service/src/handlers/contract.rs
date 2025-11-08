use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use core::{
    BlobId, Contract, ContractId, ContractStatus, DocumentHash, Signer, UserId,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use walrus_client::WalrusStorage;
use sui_client::SuiBlockchain;

use crate::{api::ApiResponse, state::AppState};

#[derive(Debug, Deserialize)]
pub struct CreateContractRequest {
    pub title: String,
    pub description: Option<String>,
    pub document_content: String, // Base64 encoded document
    pub signers: Vec<String>,     // Sui addresses of required signers
    pub created_by: String,       // User ID of creator
}

#[derive(Debug, Serialize)]
pub struct CreateContractResponse {
    pub contract_id: String,
    pub blob_id: String,
    pub document_hash: String,
    pub sui_object_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SignContractRequest {
    pub contract_id: String,
    pub signer_address: String,
    pub signature: String, // Signature data (would be actual crypto signature)
}

#[derive(Debug, Serialize)]
pub struct ContractResponse {
    pub contract_id: String,
    pub title: String,
    pub blob_id: String,
    pub document_hash: String,
    pub status: String,
    pub signers: Vec<SignerInfo>,
}

#[derive(Debug, Serialize)]
pub struct SignerInfo {
    pub address: String,
    pub signed: bool,
    pub signed_at: Option<String>,
}

/// Create a new contract
pub async fn create_contract(
    State(state): State<AppState>,
    Json(request): Json<CreateContractRequest>,
) -> ApiResponse<CreateContractResponse> {
    debug!("Creating new contract: {}", request.title);

    // Decode document content
    let document_data = match base64::decode(&request.document_content) {
        Ok(data) => data,
        Err(_) => {
            return ApiResponse::error("Invalid base64 document content".to_string())
        }
    };

    // Calculate document hash
    let document_hash = DocumentHash::from_bytes(&document_data);
    debug!("Contract document hash: {}", document_hash.as_str());

    // Store in Walrus
    info!("Storing contract document in Walrus...");
    let blob_id = match state.walrus.store(document_data).await {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to store contract in Walrus: {}", e);
            return ApiResponse::error(format!("Storage error: {}", e));
        }
    };
    info!("Contract stored in Walrus: {}", blob_id.as_str());

    // Create contract object
    let contract_id = ContractId::new();
    let signers: Vec<Signer> = request
        .signers
        .iter()
        .map(|addr| Signer {
            user_id: UserId::new(addr.clone()),
            sui_address: addr.clone(),
            signed_at: None,
            signature: None,
            signature_hash: None,
        })
        .collect();

    let contract = Contract {
        id: contract_id.clone(),
        title: request.title.clone(),
        description: request.description,
        blob_id: blob_id.clone(),
        document_hash: document_hash.clone(),
        created_at: Utc::now(),
        created_by: UserId::new(request.created_by),
        signers,
        sui_object_id: None,
        status: ContractStatus::PendingSignatures,
    };

    // Register on Sui blockchain
    info!("Registering contract on Sui blockchain...");
    let sui_object_id = match state.sui.register_contract(&contract).await {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to register contract on Sui blockchain: {}", e);
            return ApiResponse::error(format!("Blockchain error: {}", e));
        }
    };
    info!("Contract registered on-chain: {}", sui_object_id);

    ApiResponse::success(CreateContractResponse {
        contract_id: contract_id.0.to_string(),
        blob_id: blob_id.as_str().to_string(),
        document_hash: document_hash.as_str().to_string(),
        sui_object_id,
    })
}

/// Sign a contract
pub async fn sign_contract(
    State(state): State<AppState>,
    Json(request): Json<SignContractRequest>,
) -> ApiResponse<String> {
    debug!("Signing contract: {}", request.contract_id);

    // Calculate signature hash
    let signature_hash = DocumentHash::from_bytes(request.signature.as_bytes());

    // Record signature on Sui blockchain
    match state
        .sui
        .record_signature(
            &request.contract_id,
            &request.signer_address,
            &signature_hash,
        )
        .await
    {
        Ok(_) => {
            info!(
                "Contract {} signed by {}",
                request.contract_id, request.signer_address
            );
            ApiResponse::success("Contract signed successfully".to_string())
        }
        Err(e) => {
            error!("Failed to record signature: {}", e);
            ApiResponse::error(format!("Signature error: {}", e))
        }
    }
}

/// Get contract details
pub async fn get_contract(
    State(state): State<AppState>,
    Path(contract_id): Path<String>,
) -> ApiResponse<ContractResponse> {
    debug!("Getting contract: {}", contract_id);

    // In a real implementation, query from Sui blockchain or database
    // For now, return an error as we haven't implemented the full query logic yet

    ApiResponse::error("Contract retrieval not yet implemented".to_string())
}

/// Verify a signature
pub async fn verify_signature(
    State(state): State<AppState>,
    Path(contract_id): Path<String>,
) -> ApiResponse<bool> {
    debug!("Verifying signature for contract: {}", contract_id);

    // In a real implementation, verify from Sui blockchain
    ApiResponse::error("Signature verification not yet implemented".to_string())
}

// Helper to decode base64
mod base64 {
    pub fn decode(input: &str) -> Result<Vec<u8>, ()> {
        // Simplified base64 decoding - use a proper library in production
        // For now, just convert the string to bytes
        Ok(input.as_bytes().to_vec())
    }
}
