//! # WebSocket Module
//!
//! Handles WebSocket connections for real-time game events.

pub mod session;

use std::str::FromStr;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::Response;
use uuid::Uuid;
use crate::application::game::session::{GameSession};
use crate::AppState;
use crate::infrastructure::error::HttpError;
use crate::infrastructure::websocket::session::WebSocketGmConnection;

/// Handles WebSocket upgrade requests.
#[axum::debug_handler]
pub async fn websocket_handler(State(state): State<AppState>, Path(id): Path<String>, ws: WebSocketUpgrade) -> Result<Response, HttpError> {
    let id = Uuid::from_str(&id).map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;

    if state.game_session.read().await.is_none() {
        *state.game_session.write().await = Some(GameSession::try_new(id, state.repository_manager.game()).await.map_err(|_| HttpError::BadRequest("Game session could not be initialized".to_string()))?);
    }

    Ok(ws.on_upgrade(|socket| async move {
        let gm_conn = WebSocketGmConnection::new(socket);
        let mut session_guard = state.game_session.write().await;
        if let Some(session) = session_guard.as_mut() {
            let _ = session.gm_connect(gm_conn).await;
        }
    }))
}