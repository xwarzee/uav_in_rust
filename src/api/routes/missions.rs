use crate::api::handlers::missions;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/missions")
            .route("", web::get().to(missions::list_missions))
            .route("", web::post().to(missions::create_mission))
            .route("/{id}", web::get().to(missions::get_mission_detail))
            .route("/{id}/status", web::get().to(missions::get_mission_status))
            .route("/{id}", web::delete().to(missions::cancel_mission)),
    );
}
