use sea_orm::ColIdx;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub driver: Option<String>,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub max_connections: u32,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn get_url(&self) -> String {
        let driver = self.driver.clone().unwrap_or(String::from("mysql"));
        format!(
            "{driver}://{}:{}@{}:{}/{}",
            &self.username, &self.password, &self.host, &self.port, &self.database_name
        )
    }
}
