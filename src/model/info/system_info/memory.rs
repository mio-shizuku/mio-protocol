use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInfo {
    total_memory_mb: u64,
    used_memory_mb: u64,
    total_swap_mb: u64,
    used_swap_mb: u64,
}

impl MemoryInfo {
    pub fn new(
        total_memory_mb: u64,
        used_memory_mb: u64,
        total_swap_mb: u64,
        used_swap_mb: u64,
    ) -> Self {
        Self {
            total_memory_mb,
            used_memory_mb,
            total_swap_mb,
            used_swap_mb,
        }
    }
}
