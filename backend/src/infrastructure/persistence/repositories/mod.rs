use std::sync::Arc;
use sqlx::SqlitePool;
use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::persistence::repositories::user::SqliteUserRepository;

pub mod user;
pub mod game;

pub struct RepositoryManager {
    game: Arc<SqliteGameRepository>,
    user: Arc<SqliteUserRepository>,
}

impl RepositoryManager {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            game: Arc::new(SqliteGameRepository::new(db.clone())),
            user: Arc::new(SqliteUserRepository::new(db)),
        }
    }

    pub fn game(&self) -> Arc<SqliteGameRepository> {
        self.game.clone()
    }

    pub fn user(&self) -> Arc<SqliteUserRepository> {
        self.user.clone()
    }
}

impl Clone for RepositoryManager {
    fn clone(&self) -> Self {
        Self {
            game: self.game.clone(),
            user: self.user.clone(),
        }
    }
}