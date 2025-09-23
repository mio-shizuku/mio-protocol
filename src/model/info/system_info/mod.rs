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
    uptime_secs: u64,
    load_avg: (u64, u64, u64), // (one, five, fifteen)
    cpu: cpu::CpuInfo,
    memory: memory::MemoryInfo,
    disks: Vec<disk::DiskInfo>,
    networks: Vec<network::NetworkInfo>,
}

impl SystemInfo {
    pub fn new(
        uptime_secs: u64,
        load_avg: (u64, u64, u64),
        cpu: CpuInfo,
        memory: MemoryInfo,
        disks: Vec<DiskInfo>,
        networks: Vec<NetworkInfo>,
    ) -> Self {
        Self {
            uptime_secs,
            load_avg,
            cpu,
            memory,
            disks,
            networks,
        }
    }
}
