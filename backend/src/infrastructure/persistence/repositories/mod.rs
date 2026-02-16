//! # Repository Manager Module
//!
//! Provides centralized access to all data repositories.

use std::sync::Arc;
use sqlx::SqlitePool;
use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::persistence::repositories::user::SqliteUserRepository;

pub mod user;
pub mod game;

/// Manages access to all data repositories.
///
/// Provides a single point of access to user and game repositories.
/// All repositories share the same database connection pool.
///
/// # Usage
/// 
/// The repositories are normally cloned (the Arc) and passed to the application layer.
///
/// ```rust,ignore
/// let repo_manager = RepositoryManager::new(db_pool);
/// let user_repo = repo_manager.user();
/// let game_repo = repo_manager.game();
/// ```
pub struct RepositoryManager {
    game: Arc<SqliteGameRepository>,
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
            user: Arc::new(SqliteUserRepository::new(db)),
        }
    }

    /// Returns a reference to the game repository.
    pub fn game(&self) -> Arc<SqliteGameRepository> {
        self.game.clone()
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
            user: self.user.clone(),
        }
    }
}