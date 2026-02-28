//! # Turnkeeper Backend Library
//!
//! Re-exports the core modules so they are accessible from integration tests
//! and from the binary entry-point in `main.rs`.

#[macro_use]
pub mod macros;
pub mod util;
pub mod domain;
pub mod application;
pub mod infrastructure;

use axum::{middleware, routing::get, Router};
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use crate::infrastructure::auth::middleware::gm_auth_middleware;
use crate::infrastructure::auth::AuthManager;
use crate::infrastructure::http::get_routes;
use crate::infrastructure::persistence::repositories::RepositoryManager;
use crate::infrastructure::websocket::session_manager::GameSessionManager;
use crate::infrastructure::websocket::{websocket_handler, ws_ticket};

/// Application state shared across all HTTP handlers and WebSocket connections.
#[derive(Clone)]
pub struct AppState {
    pub repository_manager: RepositoryManager,
    pub auth_manager: AuthManager,
    pub game_session_manager: GameSessionManager,
}

/// Builds the Axum application router with all routes configured.
///
/// This is extracted so that integration tests can construct the same
/// application without binding to a TCP port.
pub fn build_app(state: AppState) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(get_routes(state.clone()))
        .route("/gm/ws/{id}", get(websocket_handler))
        .route(
            "/gm/ws/ticket/{game_id}",
            post(ws_ticket).route_layer(middleware::from_fn_with_state(
                state.clone(),
                gm_auth_middleware,
            )),
        )
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors_layer))
}
