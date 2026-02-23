//! # HTTP Module
//!
//! Defines HTTP routes and handlers for the REST API.
//!
//! ## Submodules
//!
//! * `game` - Game-related HTTP handlers
//! * `user` - User authentication handlers
//! * `gm` - GM authentication handlers

mod game;
mod user;
mod gm;

use axum::{middleware, Router};
use axum::routing::{delete, get, post};
use crate::AppState;
use crate::infrastructure::auth::middleware::gm_auth_middleware;
use crate::infrastructure::http::game::{games_create, games_delete, games_get};
use crate::infrastructure::http::gm::login as login_gm;
use crate::infrastructure::http::user::{login as login_user, register as register_user};

/// Creates and configures the HTTP router with all API routes.
///
/// # Returns
///
/// An Axum [`Router`] configured with all REST API endpoints.
pub fn get_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user/games", get(games_get))
        .route("/gm/games", get(games_get).route_layer(middleware::from_fn_with_state(state.clone(), gm_auth_middleware)))
        .route("/gm/games", post(games_create).route_layer(middleware::from_fn_with_state(state.clone(), gm_auth_middleware)))
        .route("/gm/games/{id}", delete(games_delete).route_layer(middleware::from_fn_with_state(state.clone(), gm_auth_middleware)))

        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))

        .route("/gm/login", post(login_gm))
}