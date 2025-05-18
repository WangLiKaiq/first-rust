use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub subject: Uuid,
    pub expiration: u64,
}
