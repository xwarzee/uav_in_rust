use crate::drone::Position;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PositionDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<PositionDto> for Position {
    fn from(dto: PositionDto) -> Self {
        Position::new(dto.x, dto.y, dto.z)
    }
}

impl From<Position> for PositionDto {
    fn from(pos: Position) -> Self {
        PositionDto {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(tag = "type", content = "params")]
pub enum CreateMissionRequest {
    MoveTo { target: PositionDto },
    Patrol { waypoints: Vec<PositionDto> },
    Search { center: PositionDto, radius: f64 },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MissionResponse {
    pub id: String,
    pub mission_type: String,
    pub status: String,
    pub assigned_drones: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MissionListResponse {
    pub missions: Vec<MissionResponse>,
}
