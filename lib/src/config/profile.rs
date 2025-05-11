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
    #[serde(rename = "stg1")]
    #[strum(serialize = "stg1")]
    Stg1,
    #[serde(rename = "stg2_1")]
    #[strum(serialize = "stg2_1")]
    Stg2_1,
    #[serde(rename = "stg2_2")]
    #[strum(serialize = "stg2_2")]
    Stg2_2,
}

impl Profile {
    pub fn filename(&self) -> String {
        format!("{self}.toml")
    }

    pub fn env_source(&self) -> Environment {
        get_env_source(&format!("{}_APP", self.to_string().to_uppercase()))
    }
}
