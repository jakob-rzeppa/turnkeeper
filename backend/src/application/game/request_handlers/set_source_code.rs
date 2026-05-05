use crate::{
    application::game::{error::GameApplicationError, request_handlers::GameRequestHandler},
    domain::common::identifier::Id,
};

impl GameRequestHandler {
    pub async fn set_source_code(
        &self,
        id: Id,
        source_code: String,
    ) -> Result<(), GameApplicationError> {
        if self
            .game_instance_repository
            .game_has_instances(id.clone())
            .await?
        {
            return Err(GameApplicationError::GameHasInstances);
        }

        let game = self.game_repository.get_by_id(&id).await?;

        if let Some(mut game) = game {
            game.set_source_code(source_code);
            self.game_repository.save(&game).await?;
            Ok(())
        } else {
            Err(GameApplicationError::GameNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::application::{
        common::parser::MockGameParserContract, game::contracts::MockGameRepositoryContract, game_instance::contracts::MockGameInstanceRepositoryContract
    };

    use super::*;

    #[tokio::test]
    async fn test_set_source_code_success() {
        let mut game_repository = MockGameRepositoryContract::new();
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();

        game_instance_repository
            .expect_game_has_instances()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Ok(false));

        game_repository
            .expect_get_by_id()
            .times(1)
            .returning(move |_| {
                Ok(Some(crate::domain::game::entities::game::Game::new_raw(
                    Id::new(),
                    "Test Game".to_string(),
                    "Test Description".to_string(),
                    "Old Source Code".to_string(),
                    crate::domain::common::date_time::DateTime::now(),
                    crate::domain::common::date_time::DateTime::now(),
                )))
            });

        game_repository
            .expect_save()
            .withf(move |game| game.source_code() == "New Source Code")
            .times(1)
            .returning(|_| Ok(()));

        let handler = GameRequestHandler::new(
            Arc::new(game_repository),
            Arc::new(game_instance_repository),
            Arc::new(game_root_parser),
        );

        handler
            .set_source_code(Id::new(), "New Source Code".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_set_source_code_game_has_instances() {
        let mut game_repository = MockGameRepositoryContract::new();
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();

        game_instance_repository
            .expect_game_has_instances()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Ok(true));

        game_repository.expect_get_by_id().never();
        game_repository.expect_save().never();

        let handler = GameRequestHandler::new(
            Arc::new(game_repository),
            Arc::new(game_instance_repository),
            Arc::new(game_root_parser),
        );

        let result = handler
            .set_source_code(Id::new(), "New Source Code".to_string())
            .await
            .unwrap_err();

        assert!(matches!(result, GameApplicationError::GameHasInstances));
    }
}
