//! # Game Session Manager
//!
//! Manages the lifecycle of [`GameSession`] instances. Sessions are created
//! on demand when a GM or user connects and are kept alive in memory.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::session::GameSession;
use crate::AppState;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::websocket::gm_connection::{WebSocketConnection};

/// Thread-safe manager that holds all active game sessions.
///
/// Wraps a `HashMap` of game ID → [`GameSession`] behind an `Arc<RwLock<..>>`
/// so it can be shared across Axum handlers.
pub struct GameSessionManager {
    sessions: Arc<RwLock<HashMap<Uuid, Arc<GameSession<WebSocketConnection, SqliteGameRepository>>>>>
}

impl GameSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Returns an existing session for the given game, if one exists.
    pub async fn get_session(&self, game_id: Uuid) -> Option<Arc<GameSession<WebSocketConnection, SqliteGameRepository>>> {
        let sessions = self.sessions.read().await;
        sessions.get(&game_id).cloned()
    }

    /// Returns an existing session or creates a new one for the given game.
    ///
    /// # Errors
    ///
    /// Returns [`GameErrorKind::GameSessionCreationFailed`] if the game
    /// metadata cannot be loaded from the repository.
    pub async fn get_or_create_session(&self, game_id: Uuid, app_state: AppState) -> Result<Arc<GameSession<WebSocketConnection, SqliteGameRepository>>, GameError> {
        if let Some(session) = self.get_session(game_id).await {
            return Ok(session);
        }

        let mut sessions = self.sessions.write().await;

        if sessions.contains_key(&game_id) {
            return Err(GameError::new(GameErrorKind::GameSessionAlreadyExists));
        }

        let session = Arc::new(GameSession::try_new(game_id, app_state.repository_manager.game()).await?);

        sessions.insert(game_id, session.clone());

        Ok(session)
    }
}

impl Clone for GameSessionManager {
    fn clone(&self) -> Self {
        Self {
            sessions: Arc::clone(&self.sessions),
        }
    }
}