use axum::{
    extract::{Path, Query, State, WebSocketUpgrade, ws::Message},
    response::Response,
};
use serde::Deserialize;

use crate::{
    AppState, application::game_instance::{commands::GameSessionCommand, dto::IncomingMessageDto}, domain::common::identifier::Id, infrastructure::error::HttpError
};

#[derive(Deserialize)]
pub struct UserWsQueryParams {
    ticket: Option<String>,
}

/// Handles WebSocket upgrade requests.
///
/// Requires a valid `?ticket=...` query parameter obtained from `POST /user/ws/ticket/{game_id}`.
#[axum::debug_handler]
pub async fn game_session_websocket_handler(
    State(state): State<AppState>,
    Path((_game_id, game_instance_id)): Path<(String, String)>,
    Query(params): Query<UserWsQueryParams>,
    ws: WebSocketUpgrade,
) -> Result<Response, HttpError> {
    let id = Id::parse_str(&game_instance_id)
        .map_err(|_| HttpError::BadRequest("Invalid game instance id".to_string()))?;
    
    let ticket = params
        .ticket
        .ok_or_else(|| HttpError::BadRequest("Missing ticket query parameter".to_string()))?;

    // Validate the ticket and retrieve the associated user ID.
    let user = state
        .ws_ticket_manager()
        .validate_ticket(&ticket)
        .await
        .ok_or_else(|| HttpError::BadRequest("Invalid ticket".to_string()))?;

    Ok(ws.on_upgrade(async move |mut socket| {
        // Make sure to DISCONNECT from the session when the WebSocket connection is closed to ensure that we don't leave any sessions running indefinitely after all users have disconnected.
        let (command_sender, mut outgoing_receiver) = state
            .game_session_manager()
            .connect_to_session(&id, user.id())
            .await
            .expect("Failed to get game session for WebSocket connection.");

        // We should only break, never return from this loop, to ensure that we always disconnect from the session in the finally block below.
        loop {
            tokio::select! {
                game_state_recv = outgoing_receiver.recv() => {
                    if let Some(game_state) = game_state_recv {
                        let msg = game_state.to_string();
                        if let Err(e) = socket.send(Message::Text(msg.into())).await {
                            println!("Failed to send to client: {}", e);
                            break;
                        }
                    } else {
                        println!("Game session ended, closing WebSocket connection.");
                        break;
                    }
                }
                command_recv = socket.recv() => {
                    match command_recv {
                        Some(Ok(Message::Text(msg))) => {
                            let command = serde_json::from_str::<GameSessionCommand>(&msg);

                            if let Ok(command) = command {
                                if let Err(e) = command_sender.send(IncomingMessageDto::Command { command, sending_user_id: user.id().clone() }) {
                                    println!("Failed to send command to game session: {}", e);
                                    break;
                                }
                            } else {
                                println!("Received unknown command: {}", msg);
                                if let Err(e) = socket.send(Message::Text(format!("Error: Unknown command: {}", msg).into())).await {
                                    println!("Failed to send to client: {}", e);
                                    break;
                                }
                            }
                        }
                        _ => break,
                    }
                }
            }
        }

        // This NEEDS to be called when the WebSocket connection is closed to ensure that we don't leave any sessions running indefinitely after all users have disconnected.
        state.game_session_manager().disconnect_from_session(&id, user.id()).await;
    }))
}
