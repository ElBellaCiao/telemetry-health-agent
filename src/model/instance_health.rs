use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InstanceHealth {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub swap_usage: f32,
    pub minute_load_avg: f64,
    pub network_rx_pps: u64,
    pub network_tx_pps: u64,
    pub timestamp: DateTime<Utc>,
}
