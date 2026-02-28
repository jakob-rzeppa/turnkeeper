//! # WebSocket Module
//!
//! Handles WebSocket connections for real-time game events.

pub mod gm_connection;

use std::str::FromStr;
use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::Response;
use backend_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::game::session::{GameSession};
use crate::AppState;
use crate::infrastructure::error::HttpError;
use crate::infrastructure::websocket::gm_connection::WebSocketGmConnection;

#[derive(Deserialize)]
pub struct WsQueryParams {
    ticket: Option<String>,
}

/// Handles WebSocket upgrade requests.
///
/// Requires a valid `?ticket=...` query parameter obtained from `POST /gm/ws/ticket/{game_id}`.
#[axum::debug_handler]
pub async fn websocket_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<WsQueryParams>,
    ws: WebSocketUpgrade,
) -> Result<Response, HttpError> {
    let id = Uuid::from_str(&id).map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;
    let ticket = params.ticket.ok_or_else(|| HttpError::BadRequest("Missing ticket query parameter".to_string()))?;

    if state.game_session.read().await.is_none() {
        *state.game_session.write().await = Some(GameSession::try_new(id, state.repository_manager.game()).await.map_err(|_| HttpError::BadRequest("Game session could not be initialized".to_string()))?);
    }

    Ok(ws.on_upgrade(|socket| async move {
        let gm_conn = WebSocketGmConnection::new(socket);
        let session_guard = state.game_session.read().await;
        if let Some(ref session) = *session_guard {
            let _ = session.gm_connect(ticket, gm_conn).await;
        }
    }))
}


#[derive(Serialize, JsonResponse, Debug)]
pub struct WsTicketResponse {
    url: String,
}

/// POST /gm/ws/ticket/{game_id}
///
/// Returns a short-lived WebSocket URL with an embedded authentication ticket.
pub async fn ws_ticket(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<String>,
) -> Result<WsTicketResponse, HttpError> {
    let game_id = Uuid::from_str(&game_id)
        .map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;

    if state.game_session.read().await.is_none() {
        *state.game_session.write().await = Some(GameSession::try_new(game_id, state.repository_manager.game()).await.map_err(|_| HttpError::BadRequest("Game session could not be initialized.".to_string()))?);
    }

    let session_guard = state.game_session.read().await;

    if let Some(ref session) = *session_guard {
        let ticket = session.gm_pre_connect().await?;

        let host = headers
            .get("host")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| HttpError::BadRequest("Missing Host header".to_string()))?;

        let url = format!("ws://{host}/gm/ws/{game_id}?ticket={ticket}");

        Ok(WsTicketResponse { url })
    } else {
        eprintln!("Failed to get game session for game_id: {}, even though we created it before, if it was none.", game_id);
        Err(HttpError::InternalServerError)
    }
}
