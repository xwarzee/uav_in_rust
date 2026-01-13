pub mod swarm;
pub mod drones;
pub mod formations;
pub mod missions;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(swarm::configure)
            .configure(drones::configure)
            .configure(formations::configure)
            .configure(missions::configure)
            .route("/health", web::get().to(health_check))
            .route("/ws/drones", web::get().to(crate::api::websocket::websocket_handler))
    );
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "uav_swarm_api",
        "version": env!("CARGO_PKG_VERSION")
    })))
}
