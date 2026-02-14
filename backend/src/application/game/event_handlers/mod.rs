use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::{GameError};
use crate::domain::game::events::GameEvent;

pub struct GameEventHandler<GameRepository: GameRepositoryContract> {
    game: Game,
    repository: GameRepository
}

impl<GameRepository: GameRepositoryContract> GameEventHandler<GameRepository> {
    pub async fn try_new(repository: GameRepository, game_id: Uuid) -> Result<Self, GameError> {
        let metadata = repository.get_metadata_by_id(game_id).await?;

        Ok(Self {
            game: Game::new(metadata.id, metadata.name),
            repository
        })
    }

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