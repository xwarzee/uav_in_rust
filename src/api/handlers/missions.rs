use crate::api::error::ApiError;
use crate::api::models::{CreateMissionRequest, MissionListResponse, MissionResponse};
use crate::api::state::AppState;
use crate::mission::MissionType;
use actix_web::{web, HttpResponse};

#[utoipa::path(
    get,
    path = "/api/missions",
    responses(
        (status = 200, description = "List of missions", body = MissionListResponse),
        (status = 500, description = "Internal error")
    ),
    tag = "missions"
)]
pub async fn list_missions(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let missions: Vec<MissionResponse> = swarm
        .mission_executor
        .active_missions
        .iter()
        .map(|(id, mission)| {
            let mission_type = format!("{:?}", mission.mission_type);
            let status_str = match &mission.status {
                crate::mission::MissionStatus::NotStarted => "not_started".to_string(),
                crate::mission::MissionStatus::InProgress => "in_progress".to_string(),
                crate::mission::MissionStatus::Completed => "completed".to_string(),
                crate::mission::MissionStatus::Failed(reason) => {
                    format!("failed: {}", reason)
                }
            };

            MissionResponse {
                id: id.clone(),
                mission_type,
                status: status_str,
                assigned_drones: mission.assigned_drones.clone(),
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(MissionListResponse { missions }))
}

#[utoipa::path(
    post,
    path = "/api/missions",
    request_body = CreateMissionRequest,
    responses(
        (status = 200, description = "Mission created", body = MissionResponse),
        (status = 400, description = "Invalid mission request"),
        (status = 500, description = "Internal error")
    ),
    tag = "missions"
)]
pub async fn create_mission(
    state: web::Data<AppState>,
    req: web::Json<CreateMissionRequest>,
) -> Result<HttpResponse, ApiError> {
    let mission_id = {
        let mut swarm = state.swarm.lock().await;

        let drone_ids: Vec<String> = swarm.drones.keys().cloned().collect();

        if drone_ids.is_empty() {
            return Err(ApiError::ValidationError(
                "No drones available for mission".to_string(),
            ));
        }

        let mission_type = match req.into_inner() {
            CreateMissionRequest::MoveTo { target } => MissionType::MoveTo(target.into()),
            CreateMissionRequest::Patrol { waypoints } => {
                MissionType::Patrol(waypoints.into_iter().map(|p| p.into()).collect())
            }
            CreateMissionRequest::Search { center, radius } => {
                MissionType::Search(center.into(), radius)
            }
        };

        let mission_id = swarm
            .mission_executor
            .create_mission(mission_type.clone(), drone_ids.clone());

        swarm
            .mission_executor
            .start_mission(&mission_id)
            .map_err(|e| ApiError::Internal(e))?;

        mission_id
    };

    // Spawn task to execute mission asynchronously
    let swarm_state = state.swarm.clone();
    let mission_id_clone = mission_id.clone();
    tokio::spawn(async move {
        // Note: execute_mission is a long-running operation that holds the lock
        let result = {
            let mut swarm = swarm_state.lock().await;

            // Execute the mission using the swarm method to avoid borrow issues
            swarm.execute_mission_by_id(&mission_id_clone).await
        };

        let _ = result;
    });

    // Return mission info immediately
    let swarm = state.swarm.lock().await;
    let mission = swarm
        .mission_executor
        .active_missions
        .get(&mission_id)
        .ok_or_else(|| ApiError::MissionNotFound(mission_id.clone()))?;

    let response = MissionResponse {
        id: mission_id.clone(),
        mission_type: format!("{:?}", mission.mission_type),
        status: "in_progress".to_string(),
        assigned_drones: mission.assigned_drones.clone(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/missions/{id}",
    responses(
        (status = 200, description = "Mission details", body = MissionResponse),
        (status = 404, description = "Mission not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "missions",
    params(
        ("id" = String, Path, description = "Mission ID")
    )
)]
pub async fn get_mission_detail(
    mission_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let mission = swarm
        .mission_executor
        .active_missions
        .get(mission_id.as_str())
        .ok_or_else(|| ApiError::MissionNotFound(mission_id.to_string()))?;

    let status_str = match &mission.status {
        crate::mission::MissionStatus::NotStarted => "not_started".to_string(),
        crate::mission::MissionStatus::InProgress => "in_progress".to_string(),
        crate::mission::MissionStatus::Completed => "completed".to_string(),
        crate::mission::MissionStatus::Failed(reason) => format!("failed: {}", reason),
    };

    let response = MissionResponse {
        id: mission_id.to_string(),
        mission_type: format!("{:?}", mission.mission_type),
        status: status_str,
        assigned_drones: mission.assigned_drones.clone(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/missions/{id}/status",
    responses(
        (status = 200, description = "Mission status"),
        (status = 404, description = "Mission not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "missions",
    params(
        ("id" = String, Path, description = "Mission ID")
    )
)]
pub async fn get_mission_status(
    mission_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let swarm = state.swarm.lock().await;

    let mission = swarm
        .mission_executor
        .active_missions
        .get(mission_id.as_str())
        .ok_or_else(|| ApiError::MissionNotFound(mission_id.to_string()))?;

    let status_str = match &mission.status {
        crate::mission::MissionStatus::NotStarted => "not_started",
        crate::mission::MissionStatus::InProgress => "in_progress",
        crate::mission::MissionStatus::Completed => "completed",
        crate::mission::MissionStatus::Failed(_reason) => "failed",
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "mission_id": mission_id.as_str(),
        "status": status_str,
        "current_waypoint": mission.current_waypoint,
        "total_waypoints": mission.waypoints.len()
    })))
}

#[utoipa::path(
    delete,
    path = "/api/missions/{id}",
    responses(
        (status = 200, description = "Mission cancelled"),
        (status = 404, description = "Mission not found"),
        (status = 500, description = "Internal error")
    ),
    tag = "missions",
    params(
        ("id" = String, Path, description = "Mission ID")
    )
)]
pub async fn cancel_mission(
    mission_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let mut swarm = state.swarm.lock().await;

    swarm
        .mission_executor
        .cancel_mission(mission_id.as_str())
        .map_err(|e| ApiError::MissionNotFound(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Mission {} cancelled", mission_id)
    })))
}
