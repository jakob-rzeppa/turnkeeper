use crate::{
    application::game::{contracts::GameRepositoryContract, error::GameApplicationError},
    domain::game::projections::game_metadata::GameMetadataProjection,
};
use std::sync::Arc;

pub struct OverviewGameResponse {
    pub games_metadata: Vec<GameMetadataProjection>,
}

pub struct GameListAllRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> GameListAllRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self {
        Self { repository }
    }

    pub async fn list_all(&self) -> Result<OverviewGameResponse, GameApplicationError> {
        Ok(OverviewGameResponse {
            games_metadata: self.repository.list_all().await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::domain::common::date_time::DateTime;
    use crate::domain::common::identifier::Identifier;

    #[tokio::test]
    async fn test_list_all_games_success() {
        let mut repository = MockGameRepositoryContract::new();

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
            Box::pin(async move { Ok(cloned) })
        });

        let handler = GameListAllRequestHandler::new(Arc::new(repository));
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

        repository
            .expect_list_all()
            .times(1)
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let handler = GameListAllRequestHandler::new(Arc::new(repository));
        let result = handler.list_all().await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.games_metadata.len(), 0);
    }
}
