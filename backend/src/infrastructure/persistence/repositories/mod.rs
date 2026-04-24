//! # Repository Manager Module
//!
//! Provides centralized access to all data repositories.

use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::persistence::repositories::game_instance::SqliteGameInstanceRepository;
use crate::infrastructure::persistence::repositories::user::SqliteUserRepository;
use sqlx::SqlitePool;
use std::sync::Arc;

pub mod game;
pub mod game_instance;
pub mod user;

/// Manages access to all data repositories.
///
/// Provides a single point of access to user and game repositories.
/// All repositories share the same database connection pool.
///
/// # Usage
///
/// The repositories are cloned (the Arc) and passed to the application layer.
pub struct RepositoryManager {
    game: Arc<SqliteGameRepository>,
    game_instance: Arc<SqliteGameInstanceRepository>,
    user: Arc<SqliteUserRepository>,
}

impl RepositoryManager {
    /// Creates a new repository manager.
    ///
    /// # Arguments
    ///
    /// * `db` - SQLite connection pool shared across all repositories
    pub fn new(db: SqlitePool) -> Self {
        Self {
            game: Arc::new(SqliteGameRepository::new(db.clone())),
            game_instance: Arc::new(SqliteGameInstanceRepository::new(db.clone())),
            user: Arc::new(SqliteUserRepository::new(db)),
        }
    }

    /// Returns a reference to the game repository.
    pub fn game(&self) -> Arc<SqliteGameRepository> {
        self.game.clone()
    }

    /// Returns a reference to the game instance repository.
    pub fn game_instance(&self) -> Arc<SqliteGameInstanceRepository> {
        self.game_instance.clone()
    }

    /// Returns a reference to the user repository.
    pub fn user(&self) -> Arc<SqliteUserRepository> {
        self.user.clone()
    }
}

impl Clone for RepositoryManager {
    fn clone(&self) -> Self {
        Self {
            game: self.game.clone(),
            game_instance: self.game_instance.clone(),
            user: self.user.clone(),
        }
    }
}
