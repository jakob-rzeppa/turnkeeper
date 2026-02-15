use std::sync::Arc;
use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::requests::{DeleteGameRequest};
use crate::domain::game::error::GameError;

pub struct DeleteGameRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> DeleteGameRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self { Self { repository } }

    pub async fn delete_game(&self, request: DeleteGameRequest) -> Result<(), GameError> {
        self.repository.delete(request.id).await?;

        Ok(())
    }
}