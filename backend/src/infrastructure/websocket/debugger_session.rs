use axum::{ extract::{ Path, Query, State, WebSocketUpgrade }, response::Response };
use serde::Deserialize;

use crate::{
    AppState,
    application::{ game::{ contracts::GameRepositoryContract, debugger::DebuggerSession } },
    domain::common::identifier::Id,
    infrastructure::error::HttpError,
};

#[derive(Deserialize)]
pub struct UserWsQueryParams {
    ticket: Option<String>,
}

/// Handles WebSocket upgrade requests.
///
/// Requires a valid `?ticket=...` query parameter obtained from `POST /ws/ticket/{game_id}`.
#[axum::debug_handler]
pub async fn game_debugger_session_websocket_handler(
    State(state): State<AppState>,
    Path(game_id): Path<String>,
    Query(params): Query<UserWsQueryParams>,
    ws: WebSocketUpgrade
) -> Result<Response, HttpError> {
    let id = Id::parse_str(&game_id).map_err(|_|
        HttpError::BadRequest("Invalid game id".to_string())
    )?;

    let ticket = params.ticket.ok_or_else(||
        HttpError::BadRequest("Missing ticket query parameter".to_string())
    )?;

    // Validate the ticket and retrieve the associated user ID.
    let _user = state
        .ws_ticket_manager()
        .validate_ticket(&ticket).await
        .ok_or_else(|| HttpError::BadRequest("Invalid ticket".to_string()))?;

    Ok(
        ws.on_upgrade(async move |mut socket| {
            loop {
            }

            // Since the debugger doesn't persist anything it is save to just drop it and let the task be aborted.
        })
    )
}
