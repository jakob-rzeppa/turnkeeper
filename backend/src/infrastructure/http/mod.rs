mod game;
mod user;
mod gm;

use axum::Router;
use axum::routing::{delete, get, post};
use crate::AppState;
use crate::infrastructure::http::game::{games_create, games_delete};
use crate::infrastructure::http::gm::login as login_gm;
use crate::infrastructure::http::user::{login as login_user, register as register_user};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/games", post(games_create))
        .route("/games/{id}", delete(games_delete))

        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))

        .route("/gm/login", post(login_gm))
}