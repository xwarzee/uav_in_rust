use crate::api::state::AppState;
use crate::api::websocket::session::handle_websocket_session;
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let app_state = state.get_ref().clone();

    // Spawn WebSocket session handler
    actix_web::rt::spawn(async move {
        handle_websocket_session(session, msg_stream, app_state).await;
    });

    Ok(res)
}
