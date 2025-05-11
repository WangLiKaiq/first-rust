use config::{Config, ConfigError, File};
use profile::Profile;
use serde::de::DeserializeOwned;

use crate::util::dir::get_project_root;
pub mod db;
pub mod deserialize;
pub mod env;
pub mod http;
pub mod load_env;
pub mod loader;
pub mod profile;
pub mod redis;
pub mod server;

pub fn read_config<T: DeserializeOwned>(profiles: Vec<Profile>) -> Result<T, ConfigError> {
    let config_dir = get_settings_dir()?;
    let mut builder =
        Config::builder().add_source(File::from(config_dir.join("base.toml")).required(false));

    for profile in &profiles {
        builder = builder
            .add_source(File::from(config_dir.join(profile.filename())).required(true))
            .add_source(profile.env_source());
    }

    let config = builder.build()?;
    for profile in &profiles {
        tracing::info!("Successfully read config profile: {profile}");
    }

    config.try_deserialize()
}

pub fn get_settings_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("settings"))
}

pub fn get_static_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("static"))
}
