use crate::drone::{DroneStatus, DroneStatusInfo, Position, Velocity};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct DroneListResponse {
    pub drones: Vec<DroneStatusInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DroneDetailResponse {
    pub id: String,
    pub position: Position,
    pub velocity: Velocity,
    pub status: DroneStatus,
    pub target_position: Option<Position>,
    pub formation_offset: Option<Position>,
    pub max_speed: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTargetRequest {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<UpdateTargetRequest> for Position {
    fn from(req: UpdateTargetRequest) -> Self {
        Position::new(req.x, req.y, req.z)
    }
}
