//! # Game Event Handlers
//!
//! Manages active game sessions and handles game events via WebSocket connections.

use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::{GameError};
use crate::domain::game::events::GameEvent;

/// The Session of a Game.
///
/// Each active game has a Session.
/// There may be no two Sessions for the same Game.
///
///
pub struct GameSession<GameRepository: GameRepositoryContract> {
    game: Game,
    repository: GameRepository
}

impl<GameRepository: GameRepositoryContract> GameSession<GameRepository> {
    pub async fn try_new(repository: GameRepository, game_id: Uuid) -> Result<Self, GameError> {
        let metadata = repository.get_metadata_by_id(game_id).await?;

        Ok(Self {
            game: Game::new(metadata.id, metadata.name),
            repository
        })
    }

    /// Loads and replays the game's event history.
    ///
    /// This reconstructs the game state by applying all historical events in order.
    ///
    /// # Errors
    ///
    /// Returns [`GameError`] if:
    /// - History cannot be loaded from the repository
    /// - Any event fails to apply
    pub async fn load_history(&mut self) -> Result<(), GameError> {
        let history = self.repository.get_game_history(self.game.id().clone()).await?;

        for event in history {
            self.handle(event).await?;
        }

        Ok(())
    }

    pub async fn handle(&mut self, event: GameEvent) -> Result<(), GameError> {
        Ok(())
    }
}