//! # WebSocket Module
//!
//! Handles WebSocket connections for real-time game events.

pub mod gm_connection;
pub mod session_manager;
mod user_connection;
mod user;
mod gm;

use axum::{middleware, Router};
use axum::routing::{get, post};
use crate::AppState;
use crate::infrastructure::auth::middleware::{gm_auth_middleware, user_auth_middleware};
use crate::infrastructure::websocket::gm::{gm_websocket_handler, gm_websocket_ticket};
use crate::infrastructure::websocket::user::{user_websocket_handler, user_websocket_ticket};

pub fn get_websocket_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/gm/ws/ticket/{game_id}",
            post(gm_websocket_ticket).route_layer(middleware::from_fn_with_state(
                state.clone(),
                gm_auth_middleware,
            ))
        )
        .route("/gm/ws/{id}", get(gm_websocket_handler))
        .route(
            "/user/ws/ticket/{game_id}",
            post(user_websocket_ticket).route_layer(middleware::from_fn_with_state(
                state.clone(),
                user_auth_middleware,
            ))
        )
        .route("/user/ws/{id}", get(user_websocket_handler))
}