use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub api_host: String,
    pub api_port: u16,
    pub sui_rpc_url: String,
    pub sui_package_id: String,
    pub sui_private_key: String,
    pub walrus_publisher_url: String,
    pub walrus_aggregator_url: String,
    pub walrus_epochs: u32,
    pub max_upload_size_mb: usize,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .set_default("api_host", "0.0.0.0")?
            .set_default("api_port", 8080)?
            .set_default("walrus_epochs", 5)?
            .set_default("max_upload_size_mb", 10)?
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
