use std::sync::LazyLock;

use lib::config::{env::get_profiles, read_config};
use secrecy::SecretString;

use crate::configure::app::AppConfig;

pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| read_config(get_profiles()).unwrap());

pub static AUTHORIZATION: &str = "Authorization";
pub static BEARER: &str = "Bearer ";
pub static TOKEN_SECRET: LazyLock<SecretString> =
    LazyLock::new(|| secrecy::SecretString::from("the-default-secret"));

pub static TOKEN_DURATION_SECONDS: i64 = 6 * 60 * 60;
