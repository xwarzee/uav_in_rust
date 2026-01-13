use crate::drone::DroneStatusInfo;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SwarmStatusResponse {
    pub drone_count: usize,
    pub simulation_running: bool,
    pub formation_stable: bool,
    pub drones: Vec<DroneStatusInfo>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartSimulationRequest {
    pub duration_seconds: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}
