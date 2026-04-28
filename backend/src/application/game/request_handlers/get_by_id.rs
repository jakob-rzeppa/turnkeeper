use crate::{
    application::game::{error::GameApplicationError, request_handlers::GameRequestHandler},
    domain::{common::identifier::Identifier, game::projections::game::GameProjection},
};

pub struct GameGetByIdResponse {
    pub game: GameProjection,
}

impl GameRequestHandler {
    pub async fn get_by_id(
        &self,
        id: Identifier,
    ) -> Result<GameGetByIdResponse, GameApplicationError> {
        let game = self.game_repository.get_by_id(&id).await?;

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
    use std::sync::Arc;

    use super::*;
    use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::application::game::root_parser::MockGameRootParserContract;
    use crate::domain::common::date_time::DateTime;
    use crate::domain::common::identifier::Identifier;
    use crate::domain::game::entities::game::Game;

    #[tokio::test]
    async fn test_get_game_by_id_success() {
        let mut repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameRootParserContract::new();
        let game_id = Identifier::new();

        repository
            .expect_get_by_id()
            .withf(move |_| true)
            .times(1)
            .returning(move |_| {
                Ok(Some(Game::new_raw(
                    game_id.clone(),
                    "Test Game".to_string(),
                    "Test Description".to_string(),
                    "Test Source Code".to_string(),
                    DateTime::now(),
                    DateTime::now(),
                )))
            });

        let handler = GameRequestHandler::new(Arc::new(repository), Arc::new(game_root_parser));
        let result = handler.get_by_id(game_id).await;

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
        let game_root_parser = MockGameRootParserContract::new();

        repository
            .expect_get_by_id()
            .times(1)
            .returning(|_| Ok(None));

        let handler = GameRequestHandler::new(Arc::new(repository), Arc::new(game_root_parser));
        let result = handler.get_by_id(game_id).await;

        assert!(result.is_err());
        match result {
            Err(GameApplicationError::GameNotFound) => (),
            _ => panic!("Expected GameNotFound error"),
        }
    }
}
