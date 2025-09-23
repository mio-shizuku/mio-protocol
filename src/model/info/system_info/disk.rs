use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiskInfo {
    mount_point: String,
    total_mb: u128,
    available_mb: u128,
}

impl DiskInfo {
    pub fn new(mount_point: String, total_mb: u128, available_mb: u128) -> Self {
        Self {
            mount_point,
            total_mb,
            available_mb,
        }
    }
}
