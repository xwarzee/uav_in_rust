use crate::swarm::DroneSwarm;
use crate::simulation::SimulationConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ports::EventPublisher;
use crate::api::websocket::publisher::BroadcastEventPublisher;

pub type SharedSwarmState = Arc<Mutex<DroneSwarm>>;

#[derive(Clone)]
pub struct AppState {
    pub swarm: SharedSwarmState,
    pub event_publisher: Arc<dyn EventPublisher>,
    pub simulation_config: Arc<SimulationConfig>,
}

impl AppState {
    pub fn new(swarm: DroneSwarm) -> Self {
        let event_publisher = Arc::new(BroadcastEventPublisher::new(100));
        Self {
            swarm: Arc::new(Mutex::new(swarm)),
            event_publisher,
            simulation_config: Arc::new(SimulationConfig::default()),
        }
    }

    pub fn new_with_config(swarm: DroneSwarm, config: SimulationConfig) -> Self {
        let event_publisher = Arc::new(BroadcastEventPublisher::new(100));
        Self {
            swarm: Arc::new(Mutex::new(swarm)),
            event_publisher,
            simulation_config: Arc::new(config),
        }
    }
}
