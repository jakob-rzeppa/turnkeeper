use crate::{
    application::game::{contracts::GameRepositoryContract, error::GameApplicationError},
    domain::game::projections::game::GameProjection,
};
use std::sync::Arc;

pub struct GameGetByIdResponse {
    pub game: GameProjection,
}

pub struct GameGetByIdRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> GameGetByIdRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_by_id(&self, id: String) -> Result<GameGetByIdResponse, GameApplicationError> {
        let game = self.repository.get_by_id(id.into()).await?;

        if let Some(game) = game {
            Ok(GameGetByIdResponse {
                game: game.get_projection(),
            })
        } else {
            Err(GameApplicationError::GameNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::domain::common::date_time::DateTime;
    use crate::domain::common::identifier::Identifier;
    use crate::domain::game::entities::game::Game;

    #[tokio::test]
    async fn test_get_game_by_id_success() {
        let mut repository = MockGameRepositoryContract::new();
        let game_id = Identifier::new();

        repository
            .expect_get_by_id()
            .withf(move |_| true)
            .times(1)
            .returning(move |_| {
                Box::pin(async move {
                    Ok(Some(Game::new_raw(
                        game_id.clone(),
                        "Test Game".to_string(),
                        "Test Description".to_string(),
                        "Test Source Code".to_string(),
                        DateTime::now(),
                        DateTime::now(),
                    )))
                })
            });

        let handler = GameGetByIdRequestHandler::new(Arc::new(repository));
        let result = handler.get_by_id(game_id.to_string()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.game.id, game_id);
        assert_eq!(response.game.name, "Test Game");
        assert_eq!(response.game.description, "Test Description");
        assert_eq!(response.game.source_code, "Test Source Code");
    }

    #[tokio::test]
    async fn test_get_game_by_id_not_found() {
        let mut repository = MockGameRepositoryContract::new();
        let game_id = Identifier::new();

        repository
            .expect_get_by_id()
            .times(1)
            .returning(|_| Box::pin(async { Ok(None) }));

        let handler = GameGetByIdRequestHandler::new(Arc::new(repository));
        let result = handler.get_by_id(game_id.to_string()).await;

        assert!(result.is_err());
        match result {
            Err(GameApplicationError::GameNotFound) => (),
            _ => panic!("Expected GameNotFound error"),
        }
    }
}
