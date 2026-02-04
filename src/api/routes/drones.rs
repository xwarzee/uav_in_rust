use crate::api::handlers::drones;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/drones")
            .route("", web::get().to(drones::list_drones))
            .route("/{id}", web::get().to(drones::get_drone_detail))
            .route("/{id}/status", web::get().to(drones::get_drone_status))
            .route("/{id}/target", web::put().to(drones::update_target))
            .route("/{id}/state", web::put().to(drones::update_drone_state)),
    );
}
