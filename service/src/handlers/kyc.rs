use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use chrono::Utc;
use domain::{
    BlobId, DocumentHash, DocumentType, KycDocument, UserId, VerificationStatus,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use uuid::Uuid;
use walrus_client::WalrusStorage;
use sui_client::SuiBlockchain;

use crate::{api::ApiResponse, state::AppState};

#[derive(Debug, Serialize)]
pub struct UploadKycResponse {
    pub document_id: String,
    pub blob_id: String,
    pub document_hash: String,
    pub sui_object_id: String,
}

#[derive(Debug, Serialize)]
pub struct KycStatusResponse {
    pub user_id: String,
    pub documents: Vec<KycDocumentInfo>,
}

#[derive(Debug, Serialize)]
pub struct KycDocumentInfo {
    pub id: String,
    pub document_type: String,
    pub verification_status: String,
    pub uploaded_at: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyKycRequest {
    pub document_id: String,
    pub status: String, // "approved" or "rejected"
}

/// Upload a KYC document
pub async fn upload_kyc_document(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> ApiResponse<UploadKycResponse> {
    debug!("Received KYC document upload request");

    let mut user_id: Option<String> = None;
    let mut document_type: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;

    // Parse multipart form data
    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "user_id" => {
                user_id = Some(field.text().await.unwrap_or_default());
            }
            "document_type" => {
                document_type = Some(field.text().await.unwrap_or_default());
            }
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.unwrap_or_default().to_vec());
            }
            _ => {}
        }
    }

    // Validate required fields
    let user_id = match user_id {
        Some(id) => UserId::new(id),
        None => return ApiResponse::error("user_id is required".to_string()),
    };

    let file_data = match file_data {
        Some(data) => data,
        None => return ApiResponse::error("file is required".to_string()),
    };

    let file_name = file_name.unwrap_or_else(|| "document.pdf".to_string());

    // Calculate document hash
    let document_hash = DocumentHash::from_bytes(&file_data);
    debug!("Document hash: {}", document_hash.as_str());

    // Store in Walrus
    info!("Storing document in Walrus...");
    let blob_id = match state.walrus.store(file_data).await {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to store document in Walrus: {}", e);
            return ApiResponse::error(format!("Storage error: {}", e));
        }
    };
    info!("Document stored in Walrus: {}", blob_id.as_str());

    // Register on Sui blockchain
    info!("Registering KYC document on Sui blockchain...");
    let sui_object_id = match state
        .sui
        .register_kyc_document(&user_id, &blob_id, &document_hash)
        .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to register on Sui blockchain: {}", e);
            return ApiResponse::error(format!("Blockchain error: {}", e));
        }
    };
    info!("KYC document registered on-chain: {}", sui_object_id);

    let document_id = Uuid::new_v4();

    // In a real implementation, you'd store this in a database or state management
    // For now, we'll just return the response

    ApiResponse::success(UploadKycResponse {
        document_id: document_id.to_string(),
        blob_id: blob_id.as_str().to_string(),
        document_hash: document_hash.as_str().to_string(),
        sui_object_id,
    })
}

/// Get KYC status for a user
pub async fn get_user_kyc_status(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResponse<KycStatusResponse> {
    debug!("Getting KYC status for user: {}", user_id);

    let user_id = UserId::new(user_id.clone());

    // Query Sui blockchain for user's KYC documents
    let document_ids = match state.sui.query_user_kyc_documents(&user_id).await {
        Ok(ids) => ids,
        Err(e) => {
            error!("Failed to query KYC documents: {}", e);
            return ApiResponse::error(format!("Query error: {}", e));
        }
    };

    // In a real implementation, fetch details for each document
    // For now, return empty list
    ApiResponse::success(KycStatusResponse {
        user_id: user_id.0,
        documents: vec![],
    })
}

/// Verify a KYC document
pub async fn verify_kyc_document(
    State(state): State<AppState>,
    Json(request): Json<VerifyKycRequest>,
) -> ApiResponse<String> {
    debug!("Verifying KYC document: {}", request.document_id);

    let status = match request.status.as_str() {
        "approved" => "approved",
        "rejected" => "rejected",
        _ => return ApiResponse::error("Invalid status".to_string()),
    };

    // Update status on Sui blockchain
    match state
        .sui
        .update_kyc_status(&request.document_id, status)
        .await
    {
        Ok(_) => {
            info!("KYC document {} verified as {}", request.document_id, status);
            ApiResponse::success(format!("KYC document verified: {}", status))
        }
        Err(e) => {
            error!("Failed to update KYC status: {}", e);
            ApiResponse::error(format!("Verification error: {}", e))
        }
    }
}
