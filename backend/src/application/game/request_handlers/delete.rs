use crate::{
    application::game::{error::GameApplicationError, request_handlers::GameRequestHandler},
    domain::common::identifier::Id,
};

pub struct DeleteGameRequest {
    pub id: Id,
}

impl GameRequestHandler {
    pub async fn delete(&self, request: DeleteGameRequest) -> Result<(), GameApplicationError> {
        if self
            .game_instance_repository
            .game_has_instances(request.id.clone())
            .await?
        {
            return Err(GameApplicationError::GameHasInstances);
        }

        self.game_repository.delete(&request.id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::common::parser::MockGameParserContract;
use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::application::game_instance::contracts::MockGameInstanceRepositoryContract;
    use crate::domain::common::identifier::Id;

    #[tokio::test]
    async fn test_delete_game_success() {
        let mut repository = MockGameRepositoryContract::new();
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();

        let game_id = Id::new();

        repository
            .expect_delete()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Ok(()));

        game_instance_repository
            .expect_game_has_instances()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Ok(false));

        let handler = GameRequestHandler::new(
            Arc::new(repository),
            Arc::new(game_instance_repository),
            Arc::new(game_root_parser),
        );
        let request = DeleteGameRequest { id: game_id };
        let result = handler.delete(request).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_game_with_instances() {
        let mut game_repository = MockGameRepositoryContract::new();
        let mut game_instance_repository = MockGameInstanceRepositoryContract::new();
        let game_root_parser = MockGameParserContract::new();

        let game_id = Id::new();

        game_repository.expect_delete().never();

        game_instance_repository
            .expect_game_has_instances()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Ok(true));

        let handler = GameRequestHandler::new(
            Arc::new(game_repository),
            Arc::new(game_instance_repository),
            Arc::new(game_root_parser),
        );
        let request = DeleteGameRequest { id: game_id };
        let result = handler.delete(request).await;

        assert!(result.is_err());
    }
}
