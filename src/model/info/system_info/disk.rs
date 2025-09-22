use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiskInfo {
    mount_point: String,
    total_gb: i64,
    available_gb: i64,
}

impl DiskInfo {
    pub fn new(mount_point: String, total_gb: i64, available_gb: i64) -> Self {
        Self {
            mount_point,
            total_gb,
            available_gb,
        }
    }
}
