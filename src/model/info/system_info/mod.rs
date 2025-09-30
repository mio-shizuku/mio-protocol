use serde::{Deserialize, Serialize};

pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;

#[cfg(test)]
mod test;

use cpu::CpuInfo;
use disk::DiskInfo;
use memory::MemoryInfo;
use network::NetworkInfo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "uptime")]
    uptime_secs: u64,
    #[serde(rename = "load_avg")]
    load_average_percent: (u64, u64, u64), // (one, five, fifteen)
    #[serde(rename = "cpu")]
    cpu_info: cpu::CpuInfo,
    #[serde(rename = "memory")]
    memory_info: memory::MemoryInfo,
    #[serde(rename = "disks")]
    disks: Vec<disk::DiskInfo>,
    #[serde(rename = "networks")]
    networks: Vec<network::NetworkInfo>,
}

impl SystemInfo {
    pub fn new(
        uptime_secs: u64,
        load_average_percent: (u64, u64, u64),
        cpu_info: CpuInfo,
        memory_info: MemoryInfo,
        disks: Vec<DiskInfo>,
        networks: Vec<NetworkInfo>,
    ) -> Self {
        Self {
            uptime_secs,
            load_average_percent,
            cpu_info,
            memory_info,
            disks,
            networks,
        }
    }
}
