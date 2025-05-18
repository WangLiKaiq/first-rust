use std::sync::Arc;

use lib::{
    db::{DatabaseClient, DatabaseClientExt},
    http::{ClientBuilder, HttpClient},
};

use crate::configure::app::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>,
    pub http: Arc<HttpClient>,
    pub redis_client: Arc<redis::Client>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, anyhow::Error> {
        let db = Arc::new(DatabaseClient::build_from_config(&config.database).await?);
        let http = Arc::new(HttpClient::build_from_config(&config.http)?);
        let redis_client = redis::Client::open(config.redis.get_connect_url()).unwrap();
        Ok(Self {
            config: Arc::new(config),
            db,
            http,
            redis_client: Arc::new(redis_client),
        })
    }
}
