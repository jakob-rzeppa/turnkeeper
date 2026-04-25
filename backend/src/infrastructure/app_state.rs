//! Application state shared across all HTTP handlers and WebSocket connections.
//!
//! Every service, repository etc. is defined here and constructed in `main.rs` before being injected into the application router.
//!
//! Note: All services should be designed to be shared across multiple request handlers and WebSocket connections. This means that they should not contain any mutable state that is not protected by synchronization primitives like `Mutex` or `RwLock`. Every service must implement `Clone`.

use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::{
    application::{
        game::request_handlers::GameRequestHandler,
        game_instance::request_handler::GameInstanceRequestHandler,
        user::request_handlers::UserRequestHandler,
    },
    infrastructure::{
        auth::jwt::{JwtGenerator, JwtValidator},
        persistence::repositories::{
            game::SqliteGameRepository, game_instance::SqliteGameInstanceRepository,
            user::SqliteUserRepository,
        },
    },
};

/// Application state shared across all HTTP handlers and WebSocket connections.
#[derive(Clone)]
pub struct AppState {
    game_request_handler: GameRequestHandler<SqliteGameRepository>,
    game_instance_request_handler:
        GameInstanceRequestHandler<SqliteGameInstanceRepository, SqliteGameRepository>,
    user_request_handler: UserRequestHandler<SqliteUserRepository, JwtGenerator, JwtValidator>,
}

impl AppState {
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        let sqlite_game_repository = Arc::new(SqliteGameRepository::new(db_pool.clone()));
        let sqlite_game_instance_repository =
            Arc::new(SqliteGameInstanceRepository::new(db_pool.clone()));
        let sqlite_user_repository = Arc::new(SqliteUserRepository::new(db_pool.clone()));
        let jwt_generator = Arc::new(JwtGenerator::new());
        let jwt_validator = Arc::new(JwtValidator::new());

        Self {
            game_request_handler: GameRequestHandler::new(sqlite_game_repository.clone()),
            game_instance_request_handler: GameInstanceRequestHandler::new(
                sqlite_game_instance_repository,
                sqlite_game_repository,
            ),
            user_request_handler: UserRequestHandler::new(
                sqlite_user_repository.clone(),
                jwt_generator,
                jwt_validator,
            ),
        }
    }

    pub fn game_request_handler(&self) -> GameRequestHandler<SqliteGameRepository> {
        self.game_request_handler.clone()
    }

    pub fn game_instance_request_handler(
        &self,
    ) -> GameInstanceRequestHandler<SqliteGameInstanceRepository, SqliteGameRepository> {
        self.game_instance_request_handler.clone()
    }

    pub fn user_request_handler(
        &self,
    ) -> UserRequestHandler<SqliteUserRepository, JwtGenerator, JwtValidator> {
        self.user_request_handler.clone()
    }
}
