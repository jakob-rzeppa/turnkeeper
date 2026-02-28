use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::session::GameSession;
use crate::AppState;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::infrastructure::persistence::repositories::game::SqliteGameRepository;
use crate::infrastructure::websocket::gm_connection::WebSocketGmConnection;

pub struct GameSessionManager {
    sessions: Arc<RwLock<HashMap<Uuid, GameSession<WebSocketGmConnection, SqliteGameRepository>>>>
}

impl GameSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_session(&self, game_id: Uuid) -> Option<GameSession<WebSocketGmConnection, SqliteGameRepository>> {
        let sessions = self.sessions.read().await;
        sessions.get(&game_id).cloned()
    }

    pub async fn get_or_create_session(&self, game_id: Uuid, app_state: AppState) -> Result<GameSession<WebSocketGmConnection, SqliteGameRepository>, GameError> {
        if let Some(session) = self.get_session(game_id).await {
            return Ok(session);
        }

        let mut sessions = self.sessions.write().await;

        if sessions.contains_key(&game_id) {
            return Err(GameError::new(GameErrorKind::GameAlreadyExists));
        }

        let session = GameSession::try_new(game_id, app_state.repository_manager.game()).await.map_err(|_| GameError::new(GameErrorKind::GameSessionCreationFailed))?;

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