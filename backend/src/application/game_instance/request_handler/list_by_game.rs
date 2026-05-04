use crate::{
    application::game_instance::{
        error::GameInstanceApplicationError, request_handler::GameInstanceRequestHandler,
    },
    domain::{
        common::identifier::Identifier,
        game::projections::game_instance_metadata::GameInstanceMetadataProjection,
    },
};

pub struct GameInstanceListByGameRequest {
    pub game_id: Identifier,
}

pub struct GameInstanceListByGameResponse {
    pub games_metadata: Vec<GameInstanceMetadataProjection>,
}

impl GameInstanceRequestHandler {
    pub async fn list_all_games(
        &self,
        request: GameInstanceListByGameRequest,
    ) -> Result<GameInstanceListByGameResponse, GameInstanceApplicationError> {
        Ok(GameInstanceListByGameResponse {
            games_metadata: self
                .game_instance_repository
                .list_by_game_id(request.game_id)
                .await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::common::parser::MockGameParserContract;
use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::application::game_instance::contracts::MockGameInstanceRepositoryContract;
    use crate::domain::common::date_time::DateTime;

    #[tokio::test]
    async fn test_list_game_instances_success() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();
        let game_id = Identifier::new();

        let instances_metadata = vec![
            GameInstanceMetadataProjection {
                id: Identifier::new(),
                name: "Instance 1".to_string(),
                game_id: game_id.clone(),
                player_count: 3,
                current_round: 1,
                gm_user_id: Identifier::new(),
                created_at: DateTime::now(),
                last_played_at: DateTime::now(),
            },
            GameInstanceMetadataProjection {
                id: Identifier::new(),
                name: "Instance 2".to_string(),
                game_id: game_id.clone(),
                player_count: 4,
                current_round: 2,
                gm_user_id: Identifier::new(),
                created_at: DateTime::now(),
                last_played_at: DateTime::now(),
            },
        ];

        let instances_clone = instances_metadata.clone();
        game_instance_repository
            .expect_list_by_game_id()
            .times(1)
            .returning(move |_| {
                let cloned = instances_clone.clone();
                Ok(cloned)
            });

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
            Arc::new(game_root_parser),
        );
        let request = GameInstanceListByGameRequest {
            game_id: game_id.clone(),
        };
        let result = handler.list_all_games(request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.games_metadata.len(), 2);
        assert_eq!(response.games_metadata[0].name, "Instance 1");
        assert_eq!(response.games_metadata[1].name, "Instance 2");
    }

    #[tokio::test]
    async fn test_list_game_instances_empty() {
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();
        let game_id = Identifier::new();

        game_instance_repository
            .expect_list_by_game_id()
            .times(1)
            .returning(|_| Ok(vec![]));

        let handler = GameInstanceRequestHandler::new(
            Arc::new(game_instance_repository),
            Arc::new(game_repository),
            Arc::new(game_root_parser),
        );
        let request = GameInstanceListByGameRequest {
            game_id: game_id.clone(),
        };
        let result = handler.list_all_games(request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.games_metadata.len(), 0);
    }
}
