use crate::{
    application::game::{
        contracts::GameRepositoryContract, error::GameApplicationError,
        request_handlers::GameRequestHandler,
    },
    domain::common::identifier::Identifier,
};

pub struct DeleteGameRequest {
    pub id: Identifier,
}

impl<GameRepository: GameRepositoryContract> GameRequestHandler<GameRepository> {
    pub async fn delete(&self, request: DeleteGameRequest) -> Result<(), GameApplicationError> {
        self.game_repository.delete(&request.id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::game::contracts::MockGameRepositoryContract;
    use crate::domain::common::identifier::Identifier;

    #[tokio::test]
    async fn test_delete_game_success() {
        let mut repository = MockGameRepositoryContract::new();
        let game_id = Identifier::new();

        repository
            .expect_delete()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));

        let handler = GameRequestHandler::new(Arc::new(repository));
        let request = DeleteGameRequest { id: game_id };
        let result = handler.delete(request).await;

        assert!(result.is_ok());
    }
}
