use axum::{
    extract::{Path, Query, State},
    Json,
};
use domain::{
    BlobId, ContractInstance, ContractTemplate, InstanceId, InstanceStatus, TemplateCategory,
    TemplateId, UserId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};
use walrus_client::WalrusStorage;

use crate::{
    api::ApiResponse,
    pdf::VariableSubstitutor,
    state::AppState
};

/// Request to create a new template
#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: String,
    pub template_pdf: String,           // Base64 encoded PDF
    pub category: String,
    pub variables: Vec<String>,
    pub price_per_use: u64,
    pub royalty_percentage: u8,
    pub is_public: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateTemplateResponse {
    pub template_id: String,
    pub blob_id: String,
    pub sui_object_id: String,
    pub marketplace_url: String,
}

/// Request to create instance from template
#[derive(Debug, Deserialize)]
pub struct CreateInstanceRequest {
    pub variable_data: HashMap<String, String>,
    pub required_signers: Vec<SignerRequest>,
    pub payment_coin_id: String,        // Sui coin object ID
}

#[derive(Debug, Deserialize)]
pub struct SignerRequest {
    pub sui_address: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct CreateInstanceResponse {
    pub instance_id: String,
    pub generated_blob_id: String,
    pub document_url: String,
    pub payment_tx: String,
    pub status: String,
}

/// Marketplace query parameters
#[derive(Debug, Deserialize)]
pub struct MarketplaceQuery {
    pub category: Option<String>,
    pub sort: Option<String>,           // "popular", "recent", "price"
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct MarketplaceResponse {
    pub templates: Vec<TemplateListingResponse>,
    pub total: u32,
    pub page: u32,
}

#[derive(Debug, Serialize)]
pub struct TemplateListingResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub price: u64,
    pub usage_count: u64,
    pub rating: f32,
    pub preview_url: String,
}

/// Create a new contract template
pub async fn create_template(
    State(state): State<AppState>,
    Json(request): Json<CreateTemplateRequest>,
) -> ApiResponse<CreateTemplateResponse> {
    debug!("Creating new template: {}", request.name);

    // 1. Decode base64 PDF
    let pdf_bytes = match base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &request.template_pdf
    ) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to decode base64 PDF: {}", e);
            return ApiResponse::error(format!("Invalid base64 PDF: {}", e));
        }
    };

    // 2. Extract variables from PDF
    let extracted_vars = match VariableSubstitutor::extract_variables(&pdf_bytes) {
        Ok(vars) => {
            info!("Extracted {} variables from template PDF", vars.len());
            vars
        },
        Err(e) => {
            warn!("Failed to extract variables from PDF: {}", e);
            // Use provided variables if extraction fails
            request.variables.clone()
        }
    };

    // Validate that provided variables match extracted variables
    if !request.variables.is_empty() {
        let missing: Vec<_> = request.variables.iter()
            .filter(|v| !extracted_vars.contains(v))
            .collect();
        if !missing.is_empty() {
            warn!("Provided variables not found in PDF: {:?}", missing);
        }
    }

    // 3. Store PDF in Walrus
    let blob_id = match state.walrus.store(pdf_bytes).await {
        Ok(id) => {
            info!("Template PDF stored in Walrus: {}", id.as_str());
            id
        },
        Err(e) => {
            error!("Failed to store template PDF in Walrus: {}", e);
            return ApiResponse::error(format!("Failed to store PDF: {}", e));
        }
    };

    // Parse category
    let category = match request.category.to_lowercase().as_str() {
        "mvno" => TemplateCategory::Mvno,
        "saas" => TemplateCategory::Saas,
        "nda" => TemplateCategory::Nda,
        "employment" => TemplateCategory::Employment,
        "service" => TemplateCategory::Service,
        "rental" => TemplateCategory::Rental,
        other => TemplateCategory::Other(other.to_string()),
    };

    // 4. Create template on Sui blockchain
    // TODO: Uncomment when Sui SDK is available
    /*
    let template_obj = state.sui.create_template(
        &request.name,
        &request.description,
        &blob_id,
        &extracted_vars,
        request.price_per_use,
        request.royalty_percentage,
        &category,
        request.is_public,
    ).await?;
    */

    // For now, create a mock response
    let template_id = uuid::Uuid::new_v4().to_string();
    let sui_object_id = format!("0x{}", hex::encode(&template_id.as_bytes()[..8]));

    info!(
        "Template created: {} (blob: {}, category: {:?})",
        template_id,
        blob_id.as_str(),
        category
    );

    ApiResponse::success(CreateTemplateResponse {
        template_id: template_id.clone(),
        blob_id: blob_id.as_str().to_string(),
        sui_object_id: sui_object_id.clone(),
        marketplace_url: format!("/api/v1/marketplace/templates/{}", template_id),
    })
}

/// Browse marketplace templates
pub async fn browse_marketplace(
    State(state): State<AppState>,
    Query(query): Query<MarketplaceQuery>,
) -> ApiResponse<MarketplaceResponse> {
    debug!("Browsing marketplace: {:?}", query);

    // TODO: Implement marketplace browsing
    // 1. Query Sui blockchain for public templates
    // 2. Filter by category
    // 3. Sort by specified field
    // 4. Paginate results

    ApiResponse::error("Marketplace browsing not yet implemented".to_string())
}

/// Get template details
pub async fn get_template(
    State(state): State<AppState>,
    Path(template_id): Path<String>,
) -> ApiResponse<ContractTemplate> {
    debug!("Getting template: {}", template_id);

    // TODO: Implement template retrieval
    // 1. Query from Sui blockchain
    // 2. Get metadata and variables
    // 3. Return template details

    ApiResponse::error("Template retrieval not yet implemented".to_string())
}

/// Create instance from template
pub async fn create_instance(
    State(state): State<AppState>,
    Path(template_id): Path<String>,
    Json(request): Json<CreateInstanceRequest>,
) -> ApiResponse<CreateInstanceResponse> {
    debug!("Creating instance from template: {}", template_id);

    // 1. Get template from blockchain
    // TODO: Uncomment when Sui SDK is available
    /*
    let template = match state.sui.get_template(&template_id).await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to get template {}: {}", template_id, e);
            return ApiResponse::error(format!("Template not found: {}", e));
        }
    };
    */

    // For now, use mock template blob_id (in real implementation, get from blockchain)
    // This should be the Walrus blob_id of the template PDF
    let template_blob_id = BlobId::new("mock_template_blob_abc123".to_string());

    // 2. Validate payment
    // TODO: Implement payment validation
    // For now, assume payment is valid
    info!("Payment validation skipped (mock): {}", request.payment_coin_id);

    // 3. Fetch template PDF from Walrus
    let template_pdf = match state.walrus.read(&template_blob_id).await {
        Ok(pdf) => {
            info!("Retrieved template PDF from Walrus: {} bytes", pdf.len());
            pdf
        },
        Err(e) => {
            error!("Failed to fetch template PDF from Walrus: {}", e);
            return ApiResponse::error(format!("Failed to fetch template: {}", e));
        }
    };

    // 4. Generate unsigned PDF with variable substitution
    let unsigned_pdf = match VariableSubstitutor::substitute_in_pdf(&template_pdf, &request.variable_data) {
        Ok(pdf) => {
            info!("Generated unsigned PDF: {} bytes", pdf.len());
            pdf
        },
        Err(e) => {
            error!("Failed to substitute variables in PDF: {}", e);
            return ApiResponse::error(format!("Failed to generate PDF: {}", e));
        }
    };

    // 5. Store unsigned PDF in Walrus
    let unsigned_blob_id = match state.walrus.store(unsigned_pdf).await {
        Ok(id) => {
            info!("Unsigned PDF stored in Walrus: {}", id.as_str());
            id
        },
        Err(e) => {
            error!("Failed to store unsigned PDF in Walrus: {}", e);
            return ApiResponse::error(format!("Failed to store PDF: {}", e));
        }
    };

    // 6. Create instance on Sui blockchain
    // TODO: Uncomment when Sui SDK is available
    /*
    let instance_obj = state.sui.create_instance(
        &template_id,
        &unsigned_blob_id,
        &request.variable_data,
        &request.required_signers,
        &request.payment_coin_id,
    ).await?;
    */

    // For now, create mock response
    let instance_id = uuid::Uuid::new_v4().to_string();
    let payment_tx = format!("0x{}", hex::encode(&instance_id.as_bytes()[..8]));

    info!(
        "Instance created: {} (unsigned blob: {})",
        instance_id,
        unsigned_blob_id.as_str()
    );

    ApiResponse::success(CreateInstanceResponse {
        instance_id: instance_id.clone(),
        generated_blob_id: unsigned_blob_id.as_str().to_string(),
        document_url: format!("/api/v1/instances/{}/document", instance_id),
        payment_tx,
        status: "pending_signatures".to_string(),
    })
}

/// Download instance document
pub async fn download_instance_document(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
) -> Result<Vec<u8>, String> {
    debug!("Downloading instance document: {}", instance_id);

    // TODO: Implement document download
    // 1. Get instance from blockchain
    // 2. Retrieve PDF from Walrus using blob_id
    // 3. Return binary PDF

    Err("Document download not yet implemented".to_string())
}

/// Sign an instance
#[derive(Debug, Deserialize)]
pub struct SignInstanceRequest {
    pub signer_address: String,
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct SignInstanceResponse {
    pub signed_at: String,
    pub remaining_signers: Vec<String>,
    pub fully_signed: bool,
}

pub async fn sign_instance(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
    Json(request): Json<SignInstanceRequest>,
) -> ApiResponse<SignInstanceResponse> {
    debug!("Signing instance: {} by {}", instance_id, request.signer_address);

    // 1. Get instance from blockchain
    // TODO: Uncomment when Sui SDK is available
    /*
    let mut instance = match state.sui.get_instance(&instance_id).await {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to get instance {}: {}", instance_id, e);
            return ApiResponse::error(format!("Instance not found: {}", e));
        }
    };
    */

    // 2. Validate signer is required
    // TODO: Check if signer is in required_signers list
    info!("Signer validation skipped (mock): {}", request.signer_address);

    // 3. Record signature on Sui blockchain
    // TODO: Uncomment when Sui SDK is available
    /*
    match state.sui.record_signature(
        &instance_id,
        &request.signer_address,
        &request.signature,
    ).await {
        Ok(_) => info!("Signature recorded on blockchain"),
        Err(e) => {
            error!("Failed to record signature: {}", e);
            return ApiResponse::error(format!("Failed to record signature: {}", e));
        }
    };
    */

    // 4. Check if fully signed
    // For mock, assume this signature completes the signing
    let fully_signed = true;
    let remaining_signers: Vec<String> = vec![];

    // 5. If fully signed, generate signed PDF
    if fully_signed {
        info!("Instance {} is now fully signed, generating final PDF", instance_id);

        // Get unsigned PDF blob_id (in real implementation, from blockchain)
        let unsigned_blob_id = BlobId::new("mock_unsigned_blob_xyz789".to_string());

        // Fetch unsigned PDF from Walrus
        let unsigned_pdf = match state.walrus.read(&unsigned_blob_id).await {
            Ok(pdf) => {
                info!("Retrieved unsigned PDF from Walrus: {} bytes", pdf.len());
                pdf
            },
            Err(e) => {
                error!("Failed to fetch unsigned PDF from Walrus: {}", e);
                return ApiResponse::error(format!("Failed to fetch unsigned PDF: {}", e));
            }
        };

        // Create signature data
        // In real implementation, get all signatures from blockchain
        let signatures = vec![
            crate::pdf::SignatureData {
                signer_name: "John Doe".to_string(),
                sui_address: request.signer_address.clone(),
                signed_at: chrono::Utc::now(),
                signature_hash: domain::DocumentHash::from_hex(request.signature.clone()),
                transaction_digest: format!("0x{}", hex::encode(&instance_id.as_bytes()[..16])),
            }
        ];

        // Generate signed PDF with signature blocks
        let signed_pdf = match crate::pdf::SignatureBlockGenerator::add_signatures_to_pdf(
            &unsigned_pdf,
            &signatures
        ) {
            Ok(pdf) => {
                info!("Generated signed PDF: {} bytes", pdf.len());
                pdf
            },
            Err(e) => {
                error!("Failed to generate signed PDF: {}", e);
                return ApiResponse::error(format!("Failed to generate signed PDF: {}", e));
            }
        };

        // Store signed PDF in Walrus
        let signed_blob_id = match state.walrus.store(signed_pdf).await {
            Ok(id) => {
                info!("Signed PDF stored in Walrus: {}", id.as_str());
                id
            },
            Err(e) => {
                error!("Failed to store signed PDF in Walrus: {}", e);
                return ApiResponse::error(format!("Failed to store signed PDF: {}", e));
            }
        };

        // Update instance on blockchain with signed blob_id
        // TODO: Uncomment when Sui SDK is available
        /*
        match state.sui.update_instance_signed_document(
            &instance_id,
            &signed_blob_id,
        ).await {
            Ok(_) => info!("Instance updated with signed document"),
            Err(e) => {
                error!("Failed to update instance: {}", e);
                return ApiResponse::error(format!("Failed to update instance: {}", e));
            }
        };
        */

        info!(
            "Instance {} fully signed and finalized (signed blob: {})",
            instance_id,
            signed_blob_id.as_str()
        );
    }

    let signed_at = chrono::Utc::now().to_rfc3339();

    ApiResponse::success(SignInstanceResponse {
        signed_at,
        remaining_signers,
        fully_signed,
    })
}
