use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryInfo {
    total_memory_gb: i64,
    used_memory_gb: i64,
    total_swap_gb: i64,
    used_swap_gb: i64,
}

impl MemoryInfo {
    pub fn new(
        total_memory_gb: i64,
        used_memory_gb: i64,
        total_swap_gb: i64,
        used_swap_gb: i64,
    ) -> Self {
        Self {
            total_memory_gb,
            used_memory_gb,
            total_swap_gb,
            used_swap_gb,
        }
    }
}
