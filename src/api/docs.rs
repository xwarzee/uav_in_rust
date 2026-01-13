use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Swarm endpoints
        crate::api::handlers::swarm::get_swarm_status,
        crate::api::handlers::swarm::start_simulation,
        crate::api::handlers::swarm::stop_simulation,

        // Drone endpoints
        crate::api::handlers::drones::list_drones,
        crate::api::handlers::drones::get_drone_detail,
        crate::api::handlers::drones::get_drone_status,
        crate::api::handlers::drones::update_target,

        // Formation endpoints
        crate::api::handlers::formations::list_formations,
        crate::api::handlers::formations::get_current_formation,
        crate::api::handlers::formations::set_formation,
        crate::api::handlers::formations::update_separation,

        // Mission endpoints
        crate::api::handlers::missions::list_missions,
        crate::api::handlers::missions::create_mission,
        crate::api::handlers::missions::get_mission_detail,
        crate::api::handlers::missions::get_mission_status,
        crate::api::handlers::missions::cancel_mission,
    ),
    components(
        schemas(
            // Swarm models
            crate::api::models::SwarmStatusResponse,
            crate::api::models::StartSimulationRequest,
            crate::api::models::SuccessResponse,

            // Drone models
            crate::api::models::DroneListResponse,
            crate::api::models::DroneDetailResponse,
            crate::api::models::UpdateTargetRequest,
            crate::drone::DroneStatusInfo,
            crate::drone::Position,
            crate::drone::Velocity,
            crate::drone::DroneStatus,

            // Formation models
            crate::api::models::FormationListResponse,
            crate::api::models::FormationResponse,
            crate::api::models::SetFormationRequest,
            crate::api::models::UpdateSeparationRequest,

            // Mission models
            crate::api::models::MissionListResponse,
            crate::api::models::MissionResponse,
            crate::api::models::CreateMissionRequest,
            crate::api::models::PositionDto,
        )
    ),
    tags(
        (name = "swarm", description = "Swarm management operations"),
        (name = "drones", description = "Individual drone operations"),
        (name = "formations", description = "Formation control"),
        (name = "missions", description = "Mission execution")
    ),
    info(
        title = "UAV Swarm Management API",
        version = "0.1.0",
        description = "REST API for managing drone swarm operations, formations, and missions. Includes real-time WebSocket updates for drone telemetry.",
        contact(
            name = "API Support",
            email = "support@uavswarm.local"
        )
    )
)]
pub struct ApiDoc;
