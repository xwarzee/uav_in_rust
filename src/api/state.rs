use crate::swarm::DroneSwarm;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use crate::api::websocket::messages::DroneUpdate;

pub type SharedSwarmState = Arc<Mutex<DroneSwarm>>;

#[derive(Clone)]
pub struct AppState {
    pub swarm: SharedSwarmState,
    pub broadcast_tx: broadcast::Sender<DroneUpdate>,
}

impl AppState {
    pub fn new(swarm: DroneSwarm) -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        Self {
            swarm: Arc::new(Mutex::new(swarm)),
            broadcast_tx,
        }
    }
}
