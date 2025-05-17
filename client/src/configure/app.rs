use lib::config::{db::DatabaseConfig, http::HttpClientConfig};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, Clone)]
pub struct AppConfig {
    pub application: ApplicationSettings,
    pub database: DatabaseConfig,
    pub http: HttpClientConfig,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}
