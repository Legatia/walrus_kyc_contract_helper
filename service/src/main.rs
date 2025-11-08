mod api;
mod config;
mod handlers;
mod state;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber;

use crate::config::AppConfig;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,service=debug".to_string()),
        )
        .init();

    info!("Starting Walrus KYC & Contract Compliance Service");

    // Load configuration
    let config = AppConfig::load()?;
    info!("Configuration loaded successfully");

    // Initialize application state
    let state = AppState::new(config.clone()).await?;
    info!("Application state initialized");

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build API routes
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .nest("/api/v1", api_routes())
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.api_port));
    info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        // KYC routes
        .route("/kyc/upload", post(handlers::kyc::upload_kyc_document))
        .route("/kyc/:user_id", get(handlers::kyc::get_user_kyc_status))
        .route("/kyc/verify", post(handlers::kyc::verify_kyc_document))
        // Contract routes
        .route("/contract/create", post(handlers::contract::create_contract))
        .route("/contract/sign", post(handlers::contract::sign_contract))
        .route(
            "/contract/:contract_id",
            get(handlers::contract::get_contract),
        )
        .route(
            "/contract/:contract_id/verify",
            get(handlers::contract::verify_signature),
        )
        // Compliance routes
        .route(
            "/compliance/audit/:user_id",
            get(handlers::compliance::get_audit_trail),
        )
        .route(
            "/compliance/gdpr/delete",
            post(handlers::compliance::gdpr_delete_request),
        )
}
