use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkInfo {
    #[serde(rename = "interface")]
    interface_name: String,
    #[serde(rename = "rx")]
    received_mb: u64,
    #[serde(rename = "tx")]
    transmitted_mb: u64,
}

impl NetworkInfo {
    pub fn new(interface_name: String, received_mb: u64, transmitted_mb: u64) -> Self {
        Self {
            interface_name,
            received_mb,
            transmitted_mb,
        }
    }
}
