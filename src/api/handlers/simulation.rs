use actix_web::{web, HttpResponse};
use crate::api::{error::ApiError, models::simulation::*, state::AppState};
use crate::simulation::SimulationMode;

/// Get current simulation mode
///
/// Returns the current simulation mode (internal or gazebo)
#[utoipa::path(
    get,
    path = "/api/simulation/mode",
    responses(
        (status = 200, description = "Current simulation mode", body = SimulationModeResponse)
    ),
    tag = "simulation"
)]
pub async fn get_mode(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;
    let mode = swarm.get_simulation_mode();

    Ok(HttpResponse::Ok().json(SimulationModeResponse {
        mode: mode.as_str().to_string(),
    }))
}

/// Set simulation mode
///
/// Switch between internal (Rust physics) and gazebo (external simulation) modes
#[utoipa::path(
    post,
    path = "/api/simulation/mode",
    request_body = SetSimulationModeRequest,
    responses(
        (status = 200, description = "Mode changed successfully", body = ModeChangeResponse),
        (status = 400, description = "Invalid mode specified"),
        (status = 500, description = "Failed to switch mode")
    ),
    tag = "simulation"
)]
pub async fn set_mode(
    request: web::Json<SetSimulationModeRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let mode = SimulationMode::from_str(&request.mode)
        .ok_or_else(|| ApiError::BadRequest(format!("Invalid simulation mode: {}", request.mode)))?;

    let mut swarm = state.swarm.lock().await;
    let config = &state.simulation_config;

    swarm.switch_mode(mode, config).await
        .map_err(|e| ApiError::InternalError(format!("Failed to switch mode: {}", e)))?;

    Ok(HttpResponse::Ok().json(ModeChangeResponse {
        message: format!("Successfully switched to {} mode", request.mode),
        new_mode: request.mode.clone(),
    }))
}

/// Get detailed simulation status
///
/// Returns comprehensive status including mode, running state, and connection info
#[utoipa::path(
    get,
    path = "/api/simulation/status",
    responses(
        (status = 200, description = "Detailed simulation status", body = SimulationStatusResponse)
    ),
    tag = "simulation"
)]
pub async fn get_status(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;
    let config = &state.simulation_config;
    let mode = swarm.get_simulation_mode();

    let bridge_url = if mode == SimulationMode::Gazebo {
        Some(config.gazebo.bridge_url.clone())
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(SimulationStatusResponse {
        mode: mode.as_str().to_string(),
        running: swarm.simulation_running,
        engine_connected: swarm.is_engine_connected(),
        update_rate_hz: config.simulation.update_rate_hz,
        bridge_url,
    }))
}

/// Start simulation
///
/// Starts the simulation loop
#[utoipa::path(
    post,
    path = "/api/simulation/start",
    responses(
        (status = 200, description = "Simulation started"),
        (status = 400, description = "Simulation already running")
    ),
    tag = "simulation"
)]
pub async fn start_simulation(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    if swarm.simulation_running {
        return Err(ApiError::BadRequest("Simulation is already running".to_string()));
    }

    swarm.simulation_running = true;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Simulation started"
    })))
}

/// Stop simulation
///
/// Stops the simulation loop
#[utoipa::path(
    post,
    path = "/api/simulation/stop",
    responses(
        (status = 200, description = "Simulation stopped"),
        (status = 400, description = "Simulation not running")
    ),
    tag = "simulation"
)]
pub async fn stop_simulation(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    if !swarm.simulation_running {
        return Err(ApiError::BadRequest("Simulation is not running".to_string()));
    }

    swarm.simulation_running = false;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Simulation stopped"
    })))
}
