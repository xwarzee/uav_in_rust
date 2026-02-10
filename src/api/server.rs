use crate::api::docs::ApiDoc;
use crate::api::state::AppState;
use crate::swarm::DroneSwarm;
use crate::simulation::SimulationConfig;
use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "uav_swarm=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub async fn run_server(swarm: DroneSwarm, config: SimulationConfig, host: &str, port: u16) -> std::io::Result<()> {
    init_tracing();

    tracing::info!("Starting UAV Swarm API server at {}:{}", host, port);

    let state = web::Data::new(AppState::new_with_config(swarm, config));

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                let origin_str = origin.as_bytes();
                origin_str.starts_with(b"http://localhost")
                    || origin_str.starts_with(b"http://127.0.0.1")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            .configure(crate::api::routes::configure_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
