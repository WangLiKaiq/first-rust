use config::{ConfigError, Environment};
use serde::Deserialize;

use super::env::get_env_source;

#[derive(
    Debug,
    strum::Display,
    strum::EnumString,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}

impl Profile {
    pub fn filename(&self) -> String {
        format!("app-{self}.toml")
    }

    pub fn env_source(&self) -> Environment {
        get_env_source(&format!("{}_APP", self.to_string().to_uppercase()))
    }
}
