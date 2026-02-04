use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response for getting current simulation mode
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SimulationModeResponse {
    /// Current simulation mode: "internal" or "gazebo"
    pub mode: String,
}

/// Request to change simulation mode
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SetSimulationModeRequest {
    /// New simulation mode: "internal" or "gazebo"
    #[schema(example = "gazebo")]
    pub mode: String,
}

/// Response with detailed simulation status
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SimulationStatusResponse {
    /// Current simulation mode
    pub mode: String,
    /// Whether simulation is currently running
    pub running: bool,
    /// Whether the simulation engine is connected (for Gazebo mode)
    pub engine_connected: bool,
    /// Update rate in Hz
    pub update_rate_hz: f64,
    /// Gazebo bridge URL (if in Gazebo mode)
    pub bridge_url: Option<String>,
}

/// Response when mode change is successful
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ModeChangeResponse {
    /// Success message
    pub message: String,
    /// New mode that was set
    pub new_mode: String,
}

/// Request to update drone state from external source (Gazebo)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateDroneStateRequest {
    /// New position of the drone
    pub position: crate::drone::Position,
    /// New velocity of the drone
    pub velocity: crate::drone::Velocity,
}
