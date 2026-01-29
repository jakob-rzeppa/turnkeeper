pub mod game;
pub mod user;
pub mod gm;

use axum::Router;
use axum::routing::{delete, get, post};
use crate::AppState;
use crate::infrastructure::http::game::{games_create, games_delete, games_get, games_get_all};
use crate::infrastructure::http::gm::login as login_gm;
use crate::infrastructure::http::user::{login as login_user, register as register_user};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/games", get(games_get_all))
        .route("/games/{id}", get(games_get))
        .route("/games", post(games_create))
        .route("/games/{id}", delete(games_delete))

        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))

        .route("/gm/login", post(login_gm))
}