mod client;

pub use client::*;

use crate::config::http::HttpClientConfig;
pub mod tracing;
pub mod utils;

pub trait ClientBuilder: Sized {
    fn build_from_config(config: &HttpClientConfig) -> Result<Self, anyhow::Error>;
}
