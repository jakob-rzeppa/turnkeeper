//! # WebSocket Module
//!
//! Handles WebSocket connections for real-time game commands.

mod game_session;
pub mod game_session_manager;
mod ticket;

use crate::AppState;
use crate::infrastructure::auth::middleware::auth_middleware;
use crate::infrastructure::websocket::game_session::game_session_websocket_handler;
use crate::infrastructure::websocket::ticket::websocket_ticket;
use axum::routing::{ get, post };
use axum::{ Router, middleware };

pub fn get_websocket_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/ws/ticket",
            post(websocket_ticket).route_layer(
                middleware::from_fn_with_state(state.clone(), auth_middleware)
            )
        )
        .route("/games/{game_id}/instances/{instance_id}/ws", get(game_session_websocket_handler)) // No auth middleware here - the WebSocket handler will handle authentication via the ticket in the query params
}
