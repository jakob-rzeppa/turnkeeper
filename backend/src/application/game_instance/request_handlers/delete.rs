use crate::{
    application::game_instance::{
        contracts::GameInstanceRepositoryContract, error::GameInstanceApplicationError,
    },
    domain::common::identifier::Identifier,
};
use std::sync::Arc;

pub struct DeleteGameInstanceRequest {
    pub game_id: Identifier,
    pub instance_id: Identifier,
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

    pub async fn delete(
        &self,
        request: DeleteGameInstanceRequest,
    ) -> Result<(), GameInstanceApplicationError> {
        self.repository
            .delete(request.game_id, request.instance_id)
            .await?;

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
        let game_id = Identifier::new();
        let game_instance_id = Identifier::new();

        repository
            .expect_delete()
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let handler = DeleteGameInstanceRequestHandler::new(Arc::new(repository));
        let request = DeleteGameInstanceRequest {
            game_id,
            instance_id: game_instance_id,
        };
        let result = handler.delete(request).await;

        assert!(result.is_ok());
    }
}
