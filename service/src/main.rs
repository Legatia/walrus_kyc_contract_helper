mod api;
mod config;
mod handlers;
mod state;
mod pdf;

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

    info!("Starting Walrus Contract Marketplace Service");

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

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        // Template marketplace routes
        .route("/templates", post(handlers::template::create_template))
        .route(
            "/marketplace/templates",
            get(handlers::template::browse_marketplace),
        )
        .route(
            "/marketplace/templates/:template_id",
            get(handlers::template::get_template),
        )
        .route(
            "/templates/:template_id/instances",
            post(handlers::template::create_instance),
        )
        .route(
            "/instances/:instance_id/document",
            get(handlers::template::download_instance_document),
        )
        .route(
            "/instances/:instance_id/sign",
            post(handlers::template::sign_instance),
        )
        // Legacy contract routes (for backward compatibility)
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
}
