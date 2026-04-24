use crate::{
    application::game_instance::{
        contracts::GameInstanceRepositoryContract, error::GameInstanceApplicationError,
    },
    domain::common::identifier::Identifier,
};
use std::sync::Arc;

pub struct DeleteGameInstanceRequest {
    pub id: Identifier,
}

pub struct DeleteGameInstanceRequestHandler<GameInstanceRepository: GameInstanceRepositoryContract>
{
    repository: Arc<GameInstanceRepository>,
}

impl<GameInstanceRepository: GameInstanceRepositoryContract>
    DeleteGameInstanceRequestHandler<GameInstanceRepository>
{
    pub fn new(repository: Arc<GameInstanceRepository>) -> Self {
        Self { repository }
    }

    pub async fn delete_game(
        &self,
        request: DeleteGameInstanceRequest,
    ) -> Result<(), GameInstanceApplicationError> {
        self.repository.delete(request.id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::game_instance::contracts::MockGameInstanceRepositoryContract;

    #[tokio::test]
    async fn test_delete_game_instance_success() {
        let mut repository = MockGameInstanceRepositoryContract::new();
        let game_instance_id = Identifier::new();

        repository
            .expect_delete()
            .withf(move |_| true)
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));

        let handler = DeleteGameInstanceRequestHandler::new(Arc::new(repository));
        let request = DeleteGameInstanceRequest {
            id: game_instance_id,
        };
        let result = handler.delete_game(request).await;

        assert!(result.is_ok());
    }
}
