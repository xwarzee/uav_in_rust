use actix_web::web;
use crate::api::handlers::simulation;

/// Configure simulation-related routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/simulation")
            .route("/mode", web::get().to(simulation::get_mode))
            .route("/mode", web::post().to(simulation::set_mode))
            .route("/status", web::get().to(simulation::get_status))
            .route("/start", web::post().to(simulation::start_simulation))
            .route("/stop", web::post().to(simulation::stop_simulation))
    );
}
