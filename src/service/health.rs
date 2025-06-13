use crate::model::InstanceHealth;
use axum::Json;
use axum::extract::State;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use sysinfo::{Networks, System};

#[derive(Clone)]
pub struct Deps {
    pub system: Arc<Mutex<System>>,
    pub networks: Arc<Mutex<Networks>>,
}

pub async fn get_health_metrics(State(state): State<Deps>) -> Json<InstanceHealth> {
    let mut system = state.system.lock().unwrap();
    let mut networks = state.networks.lock().unwrap();

    system.refresh_all();
    networks.refresh(true);

    let health_metrics = InstanceHealth {
        cpu_usage: system.global_cpu_usage(),
        memory_usage: (system.used_memory() as f32 / system.total_memory() as f32) * 100.0,
        swap_usage: (system.used_swap() as f32 / system.total_swap() as f32) * 100.0,
        minute_load_avg: System::load_average().one,
        network_rx_pps: networks
            .iter()
            .map(|(_, network)| network.packets_received())
            .sum(),
        network_tx_pps: networks
            .iter()
            .map(|(_, network)| network.packets_transmitted())
            .sum(),
        timestamp: Utc::now(),
    };

    Json(health_metrics)
}
