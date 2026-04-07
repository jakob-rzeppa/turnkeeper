use axum::{
    extract::{Query, State, WebSocketUpgrade, ws::Message},
    response::Response,
};
use serde::Deserialize;

use crate::{
    AppState,
    application::plugin::debugger::{IncomingDebuggerMessage, PluginDebugger},
    infrastructure::error::HttpError,
};

#[derive(Deserialize)]
pub struct UserWsQueryParams {
    ticket: Option<String>,
}

/// Handles WebSocket upgrade requests.
///
/// Requires a valid `?ticket=...` query parameter obtained from `POST /user/ws/ticket/{game_id}`.
#[axum::debug_handler]
pub async fn plugin_debugger_websocket_handler(
    State(state): State<AppState>,
    Query(params): Query<UserWsQueryParams>,
    ws: WebSocketUpgrade,
) -> Result<Response, HttpError> {
    let ticket = params
        .ticket
        .ok_or_else(|| HttpError::BadRequest("Missing ticket query parameter".to_string()))?;

    // Validate the ticket and retrieve the associated user ID.
    let _user = state
        .ws_session_manager
        .connect(&ticket)
        .await
        .ok_or_else(|| HttpError::BadRequest("Invalid ticket".to_string()))?;

    Ok(ws.on_upgrade(async move |mut socket| {
        let debugger = PluginDebugger::new();

        let (incoming_sender, mut outgoing_receiver) = debugger.debug().await;

        tokio::select! {
            msg = socket.recv() => {
                if let Some(Ok(Message::Text(text))) = msg {
                    let debugger_message = match serde_json::from_str::<IncomingDebuggerMessage>(&text) {
                        Ok(msg) => msg,
                        Err(err) => {
                            socket.send(Message::Text(format!("Error parsing message: {}", err).into())).await.ok();
                            return;
                        }
                    };
                    // Forward incoming messages from the WebSocket to the debugger
                    let _ = incoming_sender.send(debugger_message);
                }
            }
            msg = outgoing_receiver.recv() => {
                if let Some(debugger_message) = msg {
                    // Forward outgoing messages from the debugger to the WebSocket
                    if let Ok(text) = serde_json::to_string(&debugger_message) {
                        socket.send(Message::Text(text.into())).await.ok();
                    }
                }
            }
        }
    }))
}
