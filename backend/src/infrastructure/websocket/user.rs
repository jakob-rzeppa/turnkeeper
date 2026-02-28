use std::str::FromStr;
use axum::Extension;
use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::Response;
use backend_derive::JsonResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::domain::user::entities::User;
use crate::infrastructure::error::HttpError;
use crate::infrastructure::websocket::user_connection::WebSocketUserConnection;

#[derive(Serialize, JsonResponse, Debug)]
pub struct UserWsTicketResponse {
    url: String,
}

/// POST /user/ws/ticket/{game_id}
///
/// Returns a short-lived WebSocket URL with an embedded authentication ticket.
pub async fn user_websocket_ticket(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    headers: HeaderMap,
    Path(game_id): Path<String>,
) -> Result<UserWsTicketResponse, HttpError> {
    let game_id = Uuid::from_str(&game_id)
        .map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;
    let user_id = user.id().clone();

    let session = state.game_session_manager.get_or_create_session(game_id, state.clone()).await?;

    let ticket = session.user_pre_connect(user).await?;

    let host = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| HttpError::BadRequest("Missing Host header".to_string()))?;

    let url = format!("ws://{host}/user/ws/{game_id}?ticket={ticket}&user_id={user_id}");

    Ok(UserWsTicketResponse { url })
}

#[derive(Deserialize)]
pub struct UserWsQueryParams {
    ticket: Option<String>,
    user_id: Option<String>,
}

/// Handles WebSocket upgrade requests.
///
/// Requires a valid `?ticket=...` query parameter obtained from `POST /user/ws/ticket/{game_id}`.
#[axum::debug_handler]
pub async fn user_websocket_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<UserWsQueryParams>,
    ws: WebSocketUpgrade,
) -> Result<Response, HttpError> {
    let id = Uuid::from_str(&id).map_err(|_| HttpError::BadRequest("Invalid game id".to_string()))?;
    let ticket = params.ticket.ok_or_else(|| HttpError::BadRequest("Missing ticket query parameter".to_string()))?;
    let user_id = Uuid::from_str(
        &params.user_id.ok_or_else(|| HttpError::BadRequest("Missing user_id query parameter".to_string()))?
    ).map_err(|_| HttpError::BadRequest("Invalid user_id query parameter".to_string()))?;

    Ok(ws.on_upgrade(async move |socket| {
        let user_connection = WebSocketUserConnection::new(socket);
        let session = state.game_session_manager.get_session(id).await;
        if let Some(session) = session {
            let _ = session.user_connect(user_id, ticket, user_connection).await;
        }
    }))
}