use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpuInfo {
    cpu_cores: usize,
    cpu_usage: Vec<i32>,
    cpu_freq: Vec<u64>,
    average_cpu_usage: i32,
    average_cpu_freq: u64,
}

impl CpuInfo {
    pub fn new(
        cpu_cores: usize,
        cpu_usage: Vec<i32>,
        cpu_freq: Vec<u64>,
        average_cpu_usage: i32,
        average_cpu_freq: u64,
    ) -> Self {
        Self {
            cpu_cores,
            cpu_usage,
            cpu_freq,
            average_cpu_usage,
            average_cpu_freq,
        }
    }
}
