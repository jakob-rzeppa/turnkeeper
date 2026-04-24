use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::error::GameApplicationError;
use crate::domain::common::identifier::Identifier;
use crate::domain::game::entities::game::Game;
use std::sync::Arc;

pub struct CreateGameRequest {
    pub name: String,
    pub description: String,
}

pub struct CreateGameRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> CreateGameRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self {
        Self { repository }
    }

    /// Creates a game with a generated UUID and returns the new ID.
    pub async fn create_game(
        &self,
        request: CreateGameRequest,
    ) -> Result<Identifier, GameApplicationError> {
        let game = Game::new(request.name, request.description);

        self.repository.save(&game).await?;

        Ok(game.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::game::contracts::MockGameRepositoryContract;

    #[tokio::test]
    async fn test_create_game_successfully() {
        let mut repository = MockGameRepositoryContract::new();

        let request = CreateGameRequest {
            name: "Test Game".to_string(),
            description: "A test game description".to_string(),
        };

        // First call to check if game exists should return None
        repository
            .expect_get_by_id()
            .times(1)
            .returning(|_| Box::pin(async { Ok(None) }));

        // Save should be called once
        repository
            .expect_save()
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));

        let handler = CreateGameRequestHandler::new(Arc::new(repository));
        let result = handler.create_game(request).await;

        assert!(result.is_ok());
    }
}
