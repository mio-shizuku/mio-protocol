use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkInfo {
    name: String,
    received_mb: u64,
    transmitted_mb: u64,
}

impl NetworkInfo {
    pub fn new(name: String, received_mb: u64, transmitted_mb: u64) -> Self {
        Self {
            name,
            received_mb,
            transmitted_mb,
        }
    }
}
