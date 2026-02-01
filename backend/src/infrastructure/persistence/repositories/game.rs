use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::{GameError, GameErrorKind};

pub struct InMemoryGameRepository {
    db: Arc<RwLock<Vec<Game>>>
}

impl InMemoryGameRepository {
    pub fn new(db: Arc<RwLock<Vec<Game>>>) -> Self {
        Self { db }
    }
}

impl GameRepositoryContract for InMemoryGameRepository {
    async fn save(&self, game: Game) -> Result<(), GameError> {
        let mut db = self.db.write().await;

        for g in db.iter() {
            if g.id() == game.id() || g.name() == game.name() {
                return Err(GameError::new(GameErrorKind::GameAlreadyExists));
            }
        }

        db.push(game);

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), GameError> {
        let mut db = self.db.write().await;

        let game = db.iter().position(|g| g.id() == id).ok_or_else(|| GameError::new(GameErrorKind::GameNotFound))?;
        
        db.swap_remove(game);
        
        Ok(())
    }
}