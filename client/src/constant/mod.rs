use std::sync::LazyLock;

use lib::config::{env::get_profiles, read_config};

use crate::configure::app::AppConfig;

pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| read_config(get_profiles()).unwrap());
