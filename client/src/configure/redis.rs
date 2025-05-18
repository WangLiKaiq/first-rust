#[derive(serde::Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}

impl RedisConfig {
    pub fn get_connect_url(self: &Self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
}
