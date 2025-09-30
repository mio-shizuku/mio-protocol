use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInfo {
    #[serde(rename = "ram")]
    ram_total_mb: u64,
    #[serde(rename = "ram_used")]
    ram_used_mb: u64,
    #[serde(rename = "swap")]
    swap_total_mb: u64,
    #[serde(rename = "swap_used")]
    swap_used_mb: u64,
}

impl MemoryInfo {
    pub fn new(ram_total_mb: u64, ram_used_mb: u64, swap_total_mb: u64, swap_used_mb: u64) -> Self {
        Self {
            ram_total_mb,
            ram_used_mb,
            swap_total_mb,
            swap_used_mb,
        }
    }
}
