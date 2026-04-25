use crate::{
    application::{
        game::contracts::GameRepositoryContract,
        game_instance::{
            contracts::GameInstanceRepositoryContract, error::GameInstanceApplicationError,
            request_handler::GameInstanceRequestHandler,
        },
        interpreter::parse_game,
    },
    domain::{common::identifier::Identifier, game::entities::game_instance::GameInstance},
};

pub struct GameInstanceCreateRequest {
    pub name: String,
    pub gm_user_id: Identifier,
    pub game_id: Identifier,
}

impl<GameInstanceRepository: GameInstanceRepositoryContract, GameRepository: GameRepositoryContract>
    GameInstanceRequestHandler<GameInstanceRepository, GameRepository>
{
    pub async fn create(
        &self,
        request: GameInstanceCreateRequest,
    ) -> Result<Identifier, GameInstanceApplicationError> {
        let game = self
            .game_repository
            .get_by_id(&request.game_id)
            .await?
            .ok_or_else(|| GameInstanceApplicationError::GameNotFound(request.game_id.clone()))?;

        let game_parsing_result = parse_game(game.source_code())?;

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
    use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::application::game_instance::contracts::MockGameInstanceRepositoryContract;
    use crate::application::interpreter::{GameParsingResult, parse_game_fake};

    #[tokio::test]
    async fn test_create_game_instance_successfully() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let mut game_repository = MockGameRepositoryContract::new();

        let game_id = Identifier::new();
        let gm_user_id = Identifier::new();
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
                Box::pin(async move { Ok(Some(game)) })
            });

        // Mock the game instance repository save
        game_instance_repository
            .expect_save()
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));

        // Fake the game parsing to return empty collections
        parse_game_fake::setup(|_| {
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

        let game_id = Identifier::new();
        let gm_user_id = Identifier::new();
        let request = GameInstanceCreateRequest {
            name: "Test Game Instance".to_string(),
            gm_user_id,
            game_id: game_id.clone(),
        };

        // Mock the game repository to return None
        game_repository
            .expect_get_by_id()
            .times(1)
            .returning(|_| Box::pin(async { Ok(None) }));

        // Save should never be called
        game_instance_repository.expect_save().never();

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
        );
        let result = handler.create(request).await;

        assert!(result.is_err());
        match result {
            Err(GameInstanceApplicationError::GameNotFound(id)) => assert_eq!(id, game_id),
            _ => panic!("Expected GameNotFound error"),
        }
    }
}
