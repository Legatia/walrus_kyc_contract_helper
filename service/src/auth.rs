use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// zkLogin authentication claims from JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkLoginClaims {
    pub sub: String,           // Subject (user ID from OAuth provider)
    pub aud: String,           // Audience (OAuth client ID)
    pub iss: String,           // Issuer (OAuth provider)
    pub sui_address: String,   // Computed zkLogin Sui address
    pub exp: i64,              // Expiration timestamp
}

/// Extract zkLogin claims from request headers
/// In production, verify the JWT signature and extract claims
pub async fn extract_zklogin_user(mut request: Request, next: Next) -> Response {
    // Check for Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];

            // In production, verify JWT here:
            // 1. Decode JWT
            // 2. Verify signature against OAuth provider's JWKs
            // 3. Check expiration
            // 4. Extract claims

            debug!("zkLogin token received: {}", &token[..20.min(token.len())]);

            // For MVP, we'll trust that Sui validators have verified the zkLogin proof
            // and just extract the sui_address from the token claims
            // In production, properly decode and verify the JWT

            // Store user info in request extensions for handlers to use
            let claims = decode_jwt_mock(token);
            request.extensions_mut().insert(claims);

            next.run(request).await
        }
        Some(_) => {
            warn!("Invalid Authorization header format");
            (
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header format. Expected: Bearer <token>",
            )
                .into_response()
        }
        None => {
            // No auth header - proceed without user context
            // Some endpoints may be public
            next.run(request).await
        }
    }
}

/// Mock JWT decoder for development
/// In production, use jsonwebtoken crate to properly decode and verify
fn decode_jwt_mock(token: &str) -> ZkLoginClaims {
    // For development, return mock claims
    // In production, properly decode the JWT
    ZkLoginClaims {
        sub: "mock_user_id".to_string(),
        aud: "mock_client_id".to_string(),
        iss: "https://accounts.google.com".to_string(),
        sui_address: "0x1234567890abcdef".to_string(),
        exp: chrono::Utc::now().timestamp() + 3600,
    }
}

/// Require authenticated user for endpoint
pub fn require_auth(request: &Request) -> Result<ZkLoginClaims, StatusCode> {
    request
        .extensions()
        .get::<ZkLoginClaims>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)
}
