use crate::config::Config;
use crate::service::{Deps, get_health_metrics};
use axum::Router;
use axum::routing::get;
use std::sync::{Arc, Mutex};
use sysinfo::{Networks, System};

mod config;
mod model;
mod service;

pub use model::HEALTH_ENDPOINT;
pub use model::InstanceHealth;

pub async fn run_health_agent() -> anyhow::Result<()> {
    let state = Deps {
        system: Arc::new(Mutex::new(System::new_all())),
        networks: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
    };

    let config = Config::load_from_env();

    let app = Router::new()
        .route(&format!("/{}", HEALTH_ENDPOINT), get(get_health_metrics))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;

    tokio::spawn(async move { axum::serve(listener, app).await });

    Ok(())
}
