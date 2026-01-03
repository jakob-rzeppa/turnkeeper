mod game;

use axum::Router;
use axum::routing::{get, post};
use crate::handler::game::create_game;

pub fn get_routes() -> Router {
    Router::new()
        .route("/game", get(create_game))
}