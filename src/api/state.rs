use crate::swarm::DroneSwarm;
use crate::simulation::SimulationConfig;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use crate::api::websocket::messages::DroneUpdate;

pub type SharedSwarmState = Arc<Mutex<DroneSwarm>>;

#[derive(Clone)]
pub struct AppState {
    pub swarm: SharedSwarmState,
    pub broadcast_tx: broadcast::Sender<DroneUpdate>,
    pub simulation_config: Arc<SimulationConfig>,
}

impl AppState {
    pub fn new(swarm: DroneSwarm) -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        Self {
            swarm: Arc::new(Mutex::new(swarm)),
            broadcast_tx,
            simulation_config: Arc::new(SimulationConfig::default()),
        }
    }

    pub fn new_with_config(swarm: DroneSwarm, config: SimulationConfig) -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        Self {
            swarm: Arc::new(Mutex::new(swarm)),
            broadcast_tx,
            simulation_config: Arc::new(config),
        }
    }
}
