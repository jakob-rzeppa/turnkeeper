//! # Turnkeeper Backend Library
//!
//! Re-exports the core modules so they are accessible from integration tests
//! and from the binary entry-point in `main.rs`.

pub mod util;
pub mod domain;
pub mod application;
pub mod infrastructure;

use axum::{Router};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use crate::infrastructure::auth::AuthManager;
use crate::infrastructure::http::get_routes;
use crate::infrastructure::persistence::repositories::RepositoryManager;
use crate::infrastructure::websocket::session_manager::GameSessionManager;
use crate::infrastructure::websocket::{get_websocket_routes};

/// Application state shared across all HTTP handlers and WebSocket connections.
#[derive(Clone)]
pub struct AppState {
    /// Provides access to all data repositories (user, game).
    pub repository_manager: RepositoryManager,
    /// Manages JWT generation and validation for GM and user auth.
    pub auth_manager: AuthManager,
    /// Manages active in-memory game sessions.
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
        .merge(get_websocket_routes(state.clone()))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors_layer))
}
