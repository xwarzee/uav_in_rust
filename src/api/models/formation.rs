use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FormationResponse {
    pub formation_type: String,
    pub separation_distance: f64,
    pub is_stable: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetFormationRequest {
    pub formation_type: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSeparationRequest {
    pub separation_distance: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FormationListResponse {
    pub available_formations: Vec<String>,
}
