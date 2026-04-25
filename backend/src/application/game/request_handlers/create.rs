use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::error::GameApplicationError;
use crate::application::game::request_handlers::GameRequestHandler;
use crate::domain::common::identifier::Identifier;
use crate::domain::game::entities::game::Game;

pub struct CreateGameRequest {
    pub name: String,
    pub description: String,
}

impl<GameRepository: GameRepositoryContract> GameRequestHandler<GameRepository> {
    /// Creates a game with a generated UUID and returns the new ID.
    pub async fn create(
        &self,
        request: CreateGameRequest,
    ) -> Result<Identifier, GameApplicationError> {
        let game = Game::new(request.name, request.description);

        self.game_repository.save(&game).await?;

        Ok(game.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::game::contracts::MockGameRepositoryContract;

    #[tokio::test]
    async fn test_create_game_successfully() {
        let mut repository = MockGameRepositoryContract::new();

        let request = CreateGameRequest {
            name: "Test Game".to_string(),
            description: "A test game description".to_string(),
        };

        // Save should be called once
        repository
            .expect_save()
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));

        let handler = GameRequestHandler::new(Arc::new(repository));
        let result = handler.create(request).await;

        assert!(result.is_ok());
    }
}
