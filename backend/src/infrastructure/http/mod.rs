//! # HTTP Module
//!
//! Defines HTTP routes and handlers for the REST API.
//!
//! ## Submodules
//!
//! * `game` - Game-related HTTP handlers
//! * `user` - User authentication handlers

pub mod game;
pub mod game_instance;
pub mod user;

use crate::AppState;
use crate::infrastructure::auth::middleware::auth_middleware;
use crate::infrastructure::http::game::{games_create, games_delete, games_get};
use crate::infrastructure::http::user::{list, login, register};
use axum::routing::{delete, get, post};
use axum::{Router, middleware};

/// Creates and configures the HTTP router with all API routes.
///
/// # Returns
///
/// An Axum [`Router`] configured with all REST API endpoints.
pub fn get_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/games",
            get(games_get).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/games",
            post(games_create).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/games/{id}",
            delete(games_delete).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/users",
            get(list).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        // User routes
        .route("/login", post(login))
        .route("/register", post(register))
}
