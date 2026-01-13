use crate::api::error::ApiError;
use crate::api::models::{
    FormationListResponse, FormationResponse, SetFormationRequest, UpdateSeparationRequest,
};
use crate::api::state::AppState;
use crate::formation::FormationType;
use actix_web::{web, HttpResponse};

#[utoipa::path(
    get,
    path = "/api/formations",
    responses(
        (status = 200, description = "List of available formations", body = FormationListResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "formations"
)]
pub async fn list_formations(_state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let formations = vec![
        "triangle".to_string(),
        "line".to_string(),
        "v_formation".to_string(),
    ];

    Ok(HttpResponse::Ok().json(FormationListResponse {
        available_formations: formations,
    }))
}

#[utoipa::path(
    get,
    path = "/api/formations/current",
    responses(
        (status = 200, description = "Current formation", body = FormationResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "formations"
)]
pub async fn get_current_formation(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let formation_type = format!("{:?}", swarm.formation_manager.formation_type).to_lowercase();
    let separation_distance = swarm.formation_manager.separation_distance;
    let is_stable = swarm.formation_manager.is_formation_stable(&swarm.drones);

    Ok(HttpResponse::Ok().json(FormationResponse {
        formation_type,
        separation_distance,
        is_stable,
    }))
}

#[utoipa::path(
    post,
    path = "/api/formations/current",
    request_body = SetFormationRequest,
    responses(
        (status = 200, description = "Formation changed successfully"),
        (status = 400, description = "Invalid formation type"),
        (status = 500, description = "Internal error")
    ),
    tag = "formations"
)]
pub async fn set_formation(
    state: web::Data<AppState>,
    req: web::Json<SetFormationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    let _formation_type = FormationType::from_str(&req.formation_type).ok_or_else(|| {
        ApiError::InvalidFormation(format!(
            "Invalid formation type: {}. Available: triangle, line, v_formation",
            req.formation_type
        ))
    })?;

    swarm.set_formation(&req.formation_type);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Formation changed to {}", req.formation_type)
    })))
}

#[utoipa::path(
    put,
    path = "/api/formations/separation",
    request_body = UpdateSeparationRequest,
    responses(
        (status = 200, description = "Separation distance updated"),
        (status = 400, description = "Invalid separation distance"),
        (status = 500, description = "Internal error")
    ),
    tag = "formations"
)]
pub async fn update_separation(
    state: web::Data<AppState>,
    req: web::Json<UpdateSeparationRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    if req.separation_distance < 5.0 || req.separation_distance > 50.0 {
        return Err(ApiError::ValidationError(
            "Separation distance must be between 5.0 and 50.0".to_string(),
        ));
    }

    swarm.set_separation_distance(req.separation_distance);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Separation distance updated to {}", req.separation_distance)
    })))
}
