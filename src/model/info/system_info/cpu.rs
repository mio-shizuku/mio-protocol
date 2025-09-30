use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpuInfo {
    #[serde(rename = "cores")]
    cores: usize,
    #[serde(rename = "usages")]
    usage_percents: Vec<u32>,
    #[serde(rename = "freqs")]
    frequencies: Vec<u64>,
    #[serde(rename = "usage_avg")]
    usage_average_percent: u32,
    #[serde(rename = "freq_avg")]
    frequency_average: u64,
}

impl CpuInfo {
    pub fn new(
        cores: usize,
        usage_percents: Vec<u32>,
        frequencies: Vec<u64>,
        usage_average_percent: u32,
        frequency_average: u64,
    ) -> Self {
        Self {
            cores,
            usage_percents,
            frequencies,
            usage_average_percent,
            frequency_average,
        }
    }
}
