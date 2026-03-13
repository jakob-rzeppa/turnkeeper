use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::Response;
use backend_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::domain::game::value_objects::id::Id;
use crate::infrastructure::error::HttpError;
use crate::infrastructure::websocket::gm_connection::WebSocketConnection;

#[derive(Deserialize)]
pub struct GmWsQueryParams {
    ticket: Option<String>,
}

/// Handles WebSocket upgrade requests.
///
/// Requires a valid `?ticket=...` query parameter obtained from `POST /gm/ws/ticket/{game_id}`.
#[axum::debug_handler]
pub async fn gm_websocket_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<GmWsQueryParams>,
    ws: WebSocketUpgrade,
) -> Result<Response, HttpError> {
    let id = Id::parse_str(&id).map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;
    let ticket = params.ticket.ok_or_else(|| HttpError::BadRequest("Missing ticket query parameter".to_string()))?;

    Ok(ws.on_upgrade(async move |socket| {
        let gm_conn = WebSocketConnection::new(socket);
        let session = state.game_session_manager.get_session(id).await;
        if let Some(session) = session {
            let _ = session.gm_connect(ticket, gm_conn).await;
        }
    }))
}


#[derive(Serialize, JsonResponse, Debug)]
pub struct GmWsTicketResponse {
    url: String,
}

/// POST /gm/ws/ticket/{game_id}
///
/// Returns a short-lived WebSocket URL with an embedded authentication ticket.
pub async fn gm_websocket_ticket(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(game_id): Path<String>,
) -> Result<GmWsTicketResponse, HttpError> {
    let game_id = Id::parse_str(&game_id)
        .map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;

    let session = state.game_session_manager.get_or_create_session(game_id, state.clone()).await?;

    let ticket = session.gm_pre_connect().await?;

    let host = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| HttpError::BadRequest("Missing Host header".to_string()))?;

    let url = format!("ws://{host}/gm/ws/{game_id}?ticket={ticket}");

    Ok(GmWsTicketResponse { url })
}
