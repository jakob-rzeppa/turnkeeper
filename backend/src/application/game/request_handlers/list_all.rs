use crate::{
    application::game::{error::GameApplicationError, request_handlers::GameRequestHandler},
    domain::game::projections::game_metadata::GameMetadataProjection,
};

pub struct OverviewGameResponse {
    pub games_metadata: Vec<GameMetadataProjection>,
}

impl GameRequestHandler {
    pub async fn list_all(&self) -> Result<OverviewGameResponse, GameApplicationError> {
        Ok(OverviewGameResponse {
            games_metadata: self.game_repository.list_all().await?,
        })
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

    #[tokio::test]
    async fn test_list_all_games_success() {
        let mut repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameRootParserContract::new();

        let games_metadata = vec![
            GameMetadataProjection {
                id: Identifier::new(),
                name: "Game 1".to_string(),
                description: "Description 1".to_string(),
                created_at: DateTime::parse_str("2024-01-01T00:00:00Z").unwrap(),
                updated_at: DateTime::parse_str("2024-01-01T00:00:00Z").unwrap(),
            },
            GameMetadataProjection {
                id: Identifier::new(),
                name: "Game 2".to_string(),
                description: "Description 2".to_string(),
                created_at: DateTime::parse_str("2024-01-02T00:00:00Z").unwrap(),
                updated_at: DateTime::parse_str("2024-01-02T00:00:00Z").unwrap(),
            },
        ];

        let games_metadata_clone = games_metadata.clone();
        repository.expect_list_all().times(1).returning(move || {
            let cloned = games_metadata_clone.clone();
            Ok(cloned)
        });

        let handler = GameRequestHandler::new(Arc::new(repository), Arc::new(game_root_parser));
        let result = handler.list_all().await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.games_metadata.len(), 2);
        assert_eq!(response.games_metadata[0].name, "Game 1");
        assert_eq!(response.games_metadata[1].name, "Game 2");
    }

    #[tokio::test]
    async fn test_list_all_games_empty() {
        let mut repository = MockGameRepositoryContract::new();
        let game_root_parser = MockGameRootParserContract::new();

        repository
            .expect_list_all()
            .times(1)
            .returning(|| Ok(vec![]));

        let handler = GameRequestHandler::new(Arc::new(repository), Arc::new(game_root_parser));
        let result = handler.list_all().await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.games_metadata.len(), 0);
    }
}
