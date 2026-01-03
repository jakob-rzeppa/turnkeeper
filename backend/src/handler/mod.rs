mod game;

use axum::Router;
use axum::routing::{delete, get, post};
use crate::handler::game::{games_create, games_delete, games_get, games_get_all};

pub fn get_routes() -> Router {
    Router::new()
        .route("/games", get(games_get_all))
        .route("/games/:id", get(games_get))
        .route("/games", post(games_create))
        .route("/games/:id", delete(games_delete))
}