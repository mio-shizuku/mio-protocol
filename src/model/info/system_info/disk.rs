use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiskInfo {
    #[serde(rename = "mount")]
    mount_point: String,
    #[serde(rename = "total")]
    total_mb: u64,
    #[serde(rename = "free")]
    free_mb: u64,
}

impl DiskInfo {
    pub fn new(mount_point: String, total_mb: u64, free_mb: u64) -> Self {
        Self {
            mount_point,
            total_mb,
            free_mb,
        }
    }
}
