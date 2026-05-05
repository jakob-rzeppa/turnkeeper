use crate::{
    application::game_instance::{
        error::GameInstanceApplicationError, request_handler::GameInstanceRequestHandler,
    },
    domain::{common::identifier::Id, game::entities::game_instance::GameInstance},
};

pub struct GameInstanceCreateRequest {
    pub name: String,
    pub gm_user_id: Id,
    pub game_id: Id,
}

impl GameInstanceRequestHandler {
    pub async fn create(
        &self,
        request: GameInstanceCreateRequest,
    ) -> Result<Id, GameInstanceApplicationError> {
        let game = self
            .game_repository
            .get_by_id(&request.game_id)
            .await?
            .ok_or_else(|| GameInstanceApplicationError::GameNotFound(request.game_id.clone()))?;

        let game_parsing_result = self.game_root_parser.parse_game(game.source_code())?;

        let game_instance = GameInstance::new(
            request.name,
            request.gm_user_id,
            game_parsing_result.game_stats,
            game_parsing_result.player_stats,
            game_parsing_result.actions,
            game_parsing_result.pages,
            game,
        );

        self.game_instance_repository.save(&game_instance).await?;

        Ok(game_instance.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::common::parser::{GameParsingResult, MockGameParserContract};
    use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::application::game_instance::contracts::MockGameInstanceRepositoryContract;

    #[tokio::test]
    async fn test_create_game_instance_successfully() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let mut game_repository = MockGameRepositoryContract::new();
        let mut game_root_parser = MockGameParserContract::new();

        let game_id = Id::new();
        let gm_user_id = Id::new();
        let request = GameInstanceCreateRequest {
            name: "Test Game Instance".to_string(),
            gm_user_id: gm_user_id.clone(),
            game_id: game_id.clone(),
        };

        // Mock the game repository to return a game with valid source code
        game_repository
            .expect_get_by_id()
            .withf(move |_| true)
            .times(1)
            .returning(move |_| {
                let game = crate::domain::game::entities::game::Game::new(
                    "Test Game".to_string(),
                    "Test Description".to_string(),
                );
                Ok(Some(game))
            });

        // Mock the game instance repository save
        game_instance_repository
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        // Mock the game root parser
        game_root_parser
            .expect_parse_game()
            .times(1)
            .returning(|_| {
                Ok(GameParsingResult {
                    game_stats: Vec::new(),
                    player_stats: Vec::new(),
                    actions: Vec::new(),
                    pages: Vec::new(),
                })
            });

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
            Arc::new(game_root_parser),
        );
        let result = handler.create(request).await;

        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(!id.to_string().is_empty());
    }

    #[tokio::test]
    async fn test_create_game_instance_game_not_found() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let mut game_repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();

        let game_id = Id::new();
        let gm_user_id = Id::new();
        let request = GameInstanceCreateRequest {
            name: "Test Game Instance".to_string(),
            gm_user_id,
            game_id: game_id.clone(),
        };

        // Mock the game repository to return None
        game_repository
            .expect_get_by_id()
            .times(1)
            .returning(|_| Ok(None));

        // Save should never be called
        game_instance_repository.expect_save().never();

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
            Arc::new(game_root_parser),
        );
        let result = handler.create(request).await;

        assert!(result.is_err());
        match result {
            Err(GameInstanceApplicationError::GameNotFound(id)) => assert_eq!(id, game_id),
            _ => panic!("Expected GameNotFound error"),
        }
    }
}
