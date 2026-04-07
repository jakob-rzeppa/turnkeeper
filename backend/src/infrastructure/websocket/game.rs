use axum::{
    extract::{Path, Query, State, WebSocketUpgrade, ws::Message},
    response::Response,
};
use serde::Deserialize;

use crate::{
    AppState, application::game::commands::GameCommand, domain::game::value_objects::id::Id,
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
pub async fn game_websocket_handler(
    State(state): State<AppState>,
    Path(game_id): Path<String>,
    Query(params): Query<UserWsQueryParams>,
    ws: WebSocketUpgrade,
) -> Result<Response, HttpError> {
    let id = Id::parse_str(&game_id)
        .map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;
    let ticket = params
        .ticket
        .ok_or_else(|| HttpError::BadRequest("Missing ticket query parameter".to_string()))?;

    // Validate the ticket and retrieve the associated user ID.
    let user = state
        .ws_session_manager
        .connect(&ticket)
        .await
        .ok_or_else(|| HttpError::BadRequest("Invalid ticket".to_string()))?;

    Ok(ws.on_upgrade(async move |mut socket| {
        let (command_sender, mut game_state_receiver) = state
            .game_session_manager
            .get_session(&id, user.id())
            .await
            .expect("Failed to get game session for WebSocket connection.");

        loop {
            tokio::select! {
                game_state_recv = game_state_receiver.recv() => {
                    if let Some(game_state) = game_state_recv {
                        let msg = game_state.to_string();
                        if let Err(e) = socket.send(Message::Text(msg.into())).await {
                            println!("Failed to send game state update: {}", e);
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
                            let command = serde_json::from_str::<GameCommand>(&msg);

                            if let Ok(command) = command {
                                if let Err(e) = command_sender.send(command) {
                                    println!("Failed to send command to game session: {}", e);
                                    break;
                                }
                            } else {
                                println!("Received unknown command: {}", msg);
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            }
        }
    }))
}
