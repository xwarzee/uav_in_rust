use crate::api::error::ApiError;
use crate::api::models::{DroneDetailResponse, DroneListResponse, UpdateTargetRequest, UpdateDroneStateRequest};
use crate::api::state::AppState;
use crate::api::websocket::messages::DroneUpdate;
use actix_web::{web, HttpResponse};

#[utoipa::path(
    get,
    path = "/api/drones",
    responses(
        (status = 200, description = "List of drones", body = DroneListResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "drones"
)]
pub async fn list_drones(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let drones = swarm.get_swarm_status();

    Ok(HttpResponse::Ok().json(DroneListResponse { drones }))
}

#[utoipa::path(
    get,
    path = "/api/drones/{id}",
    responses(
        (status = 200, description = "Drone details", body = DroneDetailResponse),
        (status = 404, description = "Drone not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "drones",
    params(
        ("id" = String, Path, description = "Drone ID")
    )
)]
pub async fn get_drone_detail(
    drone_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let drone = swarm
        .drones
        .get(drone_id.as_str())
        .ok_or_else(|| ApiError::DroneNotFound(drone_id.to_string()))?;

    let response = DroneDetailResponse {
        id: drone.id.clone(),
        position: drone.position,
        velocity: drone.velocity,
        status: drone.status.clone(),
        target_position: drone.target_position,
        formation_offset: drone.formation_offset,
        max_speed: drone.max_speed,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/drones/{id}/status",
    responses(
        (status = 200, description = "Drone status", body = crate::drone::DroneStatusInfo),
        (status = 404, description = "Drone not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "drones",
    params(
        ("id" = String, Path, description = "Drone ID")
    )
)]
pub async fn get_drone_status(
    drone_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let drone = swarm
        .drones
        .get(drone_id.as_str())
        .ok_or_else(|| ApiError::DroneNotFound(drone_id.to_string()))?;

    let status_info = drone.get_status_info();

    Ok(HttpResponse::Ok().json(status_info))
}

#[utoipa::path(
    put,
    path = "/api/drones/{id}/target",
    request_body = UpdateTargetRequest,
    responses(
        (status = 200, description = "Target updated successfully"),
        (status = 404, description = "Drone not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "drones",
    params(
        ("id" = String, Path, description = "Drone ID")
    )
)]
pub async fn update_target(
    drone_id: web::Path<String>,
    target: web::Json<UpdateTargetRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    let drone = swarm
        .drones
        .get_mut(drone_id.as_str())
        .ok_or_else(|| ApiError::DroneNotFound(drone_id.to_string()))?;

    let position = target.into_inner().into();
    drone.move_to(position);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Target updated for drone {}", drone_id)
    })))
}

/// Update drone state from external simulation (Gazebo)
///
/// This endpoint is used by the Gazebo bridge to push drone state updates
#[utoipa::path(
    put,
    path = "/api/drones/{id}/state",
    request_body = UpdateDroneStateRequest,
    responses(
        (status = 200, description = "Drone state updated successfully"),
        (status = 404, description = "Drone not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "drones",
    params(
        ("id" = String, Path, description = "Drone ID")
    )
)]
pub async fn update_drone_state(
    drone_id: web::Path<String>,
    state_update: web::Json<UpdateDroneStateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let mut swarm = app_state.swarm.lock().await;

    let drone = swarm
        .drones
        .get_mut(drone_id.as_str())
        .ok_or_else(|| ApiError::DroneNotFound(drone_id.to_string()))?;

    // Update drone state from Gazebo
    drone.position = state_update.position;
    drone.velocity = state_update.velocity;

    // Broadcast update via WebSocket
    let update = DroneUpdate::PositionUpdate {
        drone_id: drone_id.to_string(),
        position: drone.position,
        velocity: drone.velocity,
    };

    app_state.event_publisher.publish(update);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": format!("State updated for drone {}", drone_id)
    })))
}
