//! # WebSocket Module
//!
//! Handles WebSocket connections for real-time game commands.

mod game;
pub mod game_session_manager;
mod game_ticket;
pub mod ws_session_manager;

use crate::AppState;
use crate::infrastructure::auth::middleware::user_auth_middleware;
use crate::infrastructure::websocket::game::game_websocket_handler;
use crate::infrastructure::websocket::game_ticket::game_websocket_ticket;
use axum::routing::{get, post};
use axum::{Router, middleware};

pub fn get_websocket_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/game/ws/ticket",
            post(game_websocket_ticket).route_layer(middleware::from_fn_with_state(
                state.clone(),
                user_auth_middleware,
            )),
        )
        .route("/game/ws/{id}", get(game_websocket_handler))
}
