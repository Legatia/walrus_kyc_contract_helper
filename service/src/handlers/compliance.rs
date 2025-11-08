use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use domain::{ComplianceEvent, ComplianceEventType, ResourceType, UserId};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use uuid::Uuid;
use sui_client::SuiBlockchain;

use crate::{api::ApiResponse, state::AppState};

#[derive(Debug, Serialize)]
pub struct AuditTrailResponse {
    pub user_id: String,
    pub events: Vec<ComplianceEventInfo>,
}

#[derive(Debug, Serialize)]
pub struct ComplianceEventInfo {
    pub event_id: String,
    pub event_type: String,
    pub action: String,
    pub timestamp: String,
    pub resource_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GdprDeleteRequest {
    pub user_id: String,
    pub reason: String,
}

/// Get audit trail for a user
pub async fn get_audit_trail(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResponse<AuditTrailResponse> {
    debug!("Getting audit trail for user: {}", user_id);

    // In a real implementation, query events from Sui blockchain
    // For now, return empty trail

    ApiResponse::success(AuditTrailResponse {
        user_id: user_id.clone(),
        events: vec![],
    })
}

/// Handle GDPR deletion request
pub async fn gdpr_delete_request(
    State(state): State<AppState>,
    Json(request): Json<GdprDeleteRequest>,
) -> ApiResponse<String> {
    debug!("GDPR deletion request for user: {}", request.user_id);

    // Log compliance event
    let event = ComplianceEvent {
        id: Uuid::new_v4(),
        event_type: ComplianceEventType::GdprRequest,
        user_id: Some(UserId::new(request.user_id.clone())),
        resource_id: request.user_id.clone(),
        resource_type: ResourceType::User,
        action: "GDPR deletion request submitted".to_string(),
        metadata: serde_json::json!({
            "reason": request.reason,
        }),
        timestamp: Utc::now(),
        sui_object_id: None,
    };

    // Log to blockchain
    match state.sui.log_compliance_event(&event).await {
        Ok(object_id) => {
            info!("GDPR deletion request logged on-chain: {}", object_id);
            ApiResponse::success(format!(
                "GDPR deletion request submitted and logged: {}",
                object_id
            ))
        }
        Err(e) => {
            error!("Failed to log GDPR request: {}", e);
            ApiResponse::error(format!("Logging error: {}", e))
        }
    }
}
