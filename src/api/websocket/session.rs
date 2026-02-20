use crate::api::state::AppState;
use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt as _;

pub async fn handle_websocket_session(
    mut session: Session,
    mut stream: MessageStream,
    state: AppState,
) {
    // Subscribe to broadcast channel
    let mut rx = state.event_publisher.subscribe();

    // Spawn a task to forward updates from broadcast to WebSocket
    let mut session_clone = session.clone();
    let forward_task = actix_web::rt::spawn(async move {
        while let Ok(update) = rx.recv().await {
            let json = serde_json::to_string(&update).unwrap_or_default();
            if session_clone.text(json).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming WebSocket messages
    while let Some(Ok(msg)) = stream.next().await {
        match msg {
            Message::Ping(bytes) => {
                if session.pong(&bytes).await.is_err() {
                    break;
                }
            }
            Message::Text(_text) => {
                // Could handle commands from client here
            }
            Message::Close(reason) => {
                let _ = session.close(reason).await;
                break;
            }
            _ => {}
        }
    }

    // Clean up
    forward_task.abort();
}
