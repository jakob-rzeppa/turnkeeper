use crate::{
    application::game::{contracts::GameRepositoryContract, error::GameApplicationError},
    domain::common::identifier::Identifier,
};
use std::sync::Arc;

pub struct DeleteGameRequest {
    pub id: Identifier,
}

pub struct DeleteGameRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> DeleteGameRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self {
        Self { repository }
    }

    pub async fn delete_game(
        &self,
        request: DeleteGameRequest,
    ) -> Result<(), GameApplicationError> {
        self.repository.delete(&request.id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
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

        let handler = DeleteGameRequestHandler::new(Arc::new(repository));
        let request = DeleteGameRequest { id: game_id };
        let result = handler.delete_game(request).await;

        assert!(result.is_ok());
    }
}
