use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing_log::log;

use crate::config::db::DatabaseConfig;

pub type DatabaseClient = DatabaseConnection;
pub trait DatabaseClientExt: Sized {
    fn build_from_config(
        config: &DatabaseConfig,
    ) -> impl std::future::Future<Output = Result<Self, anyhow::Error>>;
}

impl DatabaseClientExt for DatabaseClient {
    async fn build_from_config(config: &DatabaseConfig) -> Result<Self, anyhow::Error> {
        let mut opt = ConnectOptions::new(config.get_url());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt).await?;
        Ok(db)
    }
}
