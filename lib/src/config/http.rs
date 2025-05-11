use std::time::Duration;

use super::deserialize::deserialize_duration;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpClientConfig {
    #[serde(deserialize_with = "deserialize_duration")]
    pub timeout: Duration,
}
