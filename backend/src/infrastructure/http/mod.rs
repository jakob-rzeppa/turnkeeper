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
use crate::infrastructure::error::HttpError;
use crate::infrastructure::http::game::{
    games_check_source_code, games_create, games_delete, games_get, games_get_by_id,
    games_update_source_code,
};
use crate::infrastructure::http::game_instance::{
    game_instances_delete, game_instances_get_metadata_by_game_id, game_instances_post,
};
use crate::infrastructure::http::user::{list, login, register};
use axum::routing::{delete, get, patch, post};
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
            "/games/{id}",
            get(games_get_by_id).route_layer(middleware::from_fn_with_state(
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
            "/games/{id}/source-code",
            patch(games_update_source_code).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/games/{game_id}/check",
            get(games_check_source_code).route_layer(middleware::from_fn_with_state(
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
            "/games/{game_id}/instances",
            get(game_instances_get_metadata_by_game_id).route_layer(
                middleware::from_fn_with_state(state.clone(), auth_middleware),
            ),
        )
        .route(
            "/games/{game_id}/instances",
            post(game_instances_post).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/games/{game_id}/instances/{instance_id}",
            delete(game_instances_delete).route_layer(middleware::from_fn_with_state(
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
        .fallback(fallback_handler)
}

async fn fallback_handler() -> Result<String, HttpError> {
    Err(HttpError::NotFound("Endpoint not found".to_string()))
}
