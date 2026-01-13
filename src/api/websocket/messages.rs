use crate::drone::{DroneStatus, Position, Velocity};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum DroneUpdate {
    PositionUpdate {
        drone_id: String,
        position: Position,
        velocity: Velocity,
    },
    StatusChange {
        drone_id: String,
        status: DroneStatus,
    },
    FormationUpdate {
        formation_stable: bool,
    },
    MissionProgress {
        mission_id: String,
        waypoint: usize,
    },
}
