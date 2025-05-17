use std::sync::Arc;

use lib::{
    db::{DatabaseClient, DatabaseClientExt},
    http::{ClientBuilder, HttpClient},
};

use crate::configure::app::AppConfig;

pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>,
    pub http: Arc<HttpClient>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, anyhow::Error> {
        let db = Arc::new(DatabaseClient::build_from_config(&config.database).await?);
        let http = Arc::new(HttpClient::build_from_config(&config.http)?);
        Ok(Self {
            config: Arc::new(config),
            db,
            http,
        })
    }
}
