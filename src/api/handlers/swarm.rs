use crate::api::error::ApiError;
use crate::api::models::{StartSimulationRequest, SuccessResponse, SwarmStatusResponse};
use crate::api::state::AppState;
use actix_web::{web, HttpResponse};
use std::time::Duration;
use tokio::time::sleep;
use utoipa;

#[utoipa::path(
    get,
    path = "/api/swarm",
    responses(
        (status = 200, description = "Swarm status retrieved", body = SwarmStatusResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "swarm"
)]
pub async fn get_swarm_status(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let drone_statuses = swarm.get_swarm_status();
    let formation_stable = swarm.formation_manager.is_formation_stable(&swarm.drones);

    let response = SwarmStatusResponse {
        drone_count: swarm.drones.len(),
        simulation_running: swarm.simulation_running,
        formation_stable,
        drones: drone_statuses,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/swarm/start",
    request_body = StartSimulationRequest,
    responses(
        (status = 200, description = "Simulation started", body = SuccessResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "swarm"
)]
pub async fn start_simulation(
    state: web::Data<AppState>,
    _req: web::Json<StartSimulationRequest>,
) -> Result<HttpResponse, ApiError> {
    {
        let mut swarm = state.swarm.lock().await;
        swarm.simulation_running = true;
    }

    let swarm_state = state.swarm.clone();
    let broadcast_tx = state.broadcast_tx.clone();

    tokio::spawn(async move {
        loop {
            let (should_continue, updates) = {
                let mut swarm = swarm_state.lock().await;

                if !swarm.simulation_running {
                    break;
                }

                swarm.update_swarm();
                let drone_status = swarm.get_swarm_status();
                (true, drone_status)
            };

            if !should_continue {
                break;
            }

            // Broadcast updates to WebSocket clients
            for drone_info in updates {
                use crate::api::websocket::messages::DroneUpdate;
                let _ = broadcast_tx.send(DroneUpdate::PositionUpdate {
                    drone_id: drone_info.id.clone(),
                    position: drone_info.position,
                    velocity: drone_info.velocity,
                });
            }

            sleep(Duration::from_millis(100)).await;
        }
    });

    Ok(HttpResponse::Ok().json(SuccessResponse {
        message: "Simulation started".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/swarm/stop",
    responses(
        (status = 200, description = "Simulation stopped", body = SuccessResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "swarm"
)]
pub async fn stop_simulation(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    swarm.stop_simulation();

    Ok(HttpResponse::Ok().json(SuccessResponse {
        message: "Simulation stopped".to_string(),
    }))
}
