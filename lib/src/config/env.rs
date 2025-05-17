use std::str::FromStr;

use super::profile::Profile;
use dotenvy::dotenv;

pub fn get_env_source(prefix: &str) -> config::Environment {
    config::Environment::with_prefix(prefix)
        .prefix_separator("__")
        .separator("__")
}

pub fn get_profiles() -> impl Iterator<Item = Profile> {
    load_system_properties();
    let raw = std::env::var("APP_PROFILE").unwrap_or_default();
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| Profile::from_str(s).unwrap()) // or handle Result
        .collect::<Vec<_>>() // collect to Vec to own the data
        .into_iter()
}

pub fn load_system_properties() {
    // loading the .env file from the current project root folder. client..etc..
    dotenv().ok();
}
