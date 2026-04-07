use axum::{Extension, extract::State};
use backend_derive::JsonResponse;
use serde::Serialize;

use crate::{AppState, domain::user::entities::User, infrastructure::error::HttpError};

#[derive(Serialize, JsonResponse, Debug)]
pub struct GmWsTicketResponse {
    ticket: String,
}

/// POST /ws/ticket
///
/// Returns a short-lived WebSocket URL with an embedded authentication ticket.
///
/// The ticket can be used to establish a WebSocket connection to any endpoint - the point of this is to enforce authentication and getting the user context.
///
/// The user info is needed to enforce permissions in the game session and to associate the connection to a specific user in the game.
///
/// By getting the user here first, we make sure the user is who they say they are before they can even establish a WebSocket connection, which allows us to savely allow connections and then check for the users permissions via the user_id etc. in the game session without having to worry about unauthenticated connections at all in the game session logic.
pub async fn websocket_ticket(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<GmWsTicketResponse, HttpError> {
    let ticket = state.ws_session_manager.pre_connect(user).await;

    Ok(GmWsTicketResponse { ticket })
}
