use serde::{Deserialize, Serialize};

pub mod cpu;
pub mod memory;
pub mod disk;
pub mod network;

#[cfg(test)]
mod test;

use cpu::CpuInfo;
use disk::DiskInfo;
use network::NetworkInfo;
use memory::MemoryInfo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemInfo {
    uptime_secs: u64,
    load_avg: (i64, i64, i64), // (one, five, fifteen)
    cpu: cpu::CpuInfo,
    memory: memory::MemoryInfo,
    disks: Vec<disk::DiskInfo>,
    networks: Vec<network::NetworkInfo>,
}

impl SystemInfo {
    pub fn new(
        uptime_secs: u64,
        load_avg: (i64, i64, i64),
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
