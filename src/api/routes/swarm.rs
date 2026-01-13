use crate::api::handlers::swarm;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/swarm")
            .route("", web::get().to(swarm::get_swarm_status))
            .route("/start", web::post().to(swarm::start_simulation))
            .route("/stop", web::post().to(swarm::stop_simulation)),
    );
}
