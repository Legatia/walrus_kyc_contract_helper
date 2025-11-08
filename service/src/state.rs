use anyhow::Result;
use std::sync::Arc;
use walrus_client::WalrusClient;
use sui_client::SuiClient;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub walrus: Arc<WalrusClient>,
    pub sui: Arc<SuiClient>,
    pub config: AppConfig,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self> {
        // Initialize Walrus client
        let walrus = WalrusClient::new(
            config.walrus_publisher_url.clone(),
            config.walrus_aggregator_url.clone(),
            config.walrus_epochs,
        );

        // Initialize Sui client
        let sui = SuiClient::new(
            config.sui_rpc_url.clone(),
            config.sui_package_id.clone(),
        );

        // Test Sui connection
        sui.init().await?;

        Ok(Self {
            walrus: Arc::new(walrus),
            sui: Arc::new(sui),
            config,
        })
    }
}
