use crate::api::handlers::formations;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/formations")
            .route("", web::get().to(formations::list_formations))
            .route("/current", web::get().to(formations::get_current_formation))
            .route("/current", web::post().to(formations::set_formation))
            .route("/separation", web::put().to(formations::update_separation)),
    );
}
