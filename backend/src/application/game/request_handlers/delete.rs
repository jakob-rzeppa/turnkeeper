use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::requests::{DeleteGameRequest};
use crate::domain::error::Error;

pub struct DeleteRequestHandler<GameRepository: GameRepositoryContract + 'static> {
    repository: GameRepository,
}

impl<GameRepository: GameRepositoryContract + 'static> DeleteRequestHandler<GameRepository> {
    pub fn new(repository: GameRepository) -> Self { Self { repository } }

    pub async fn create_game(&self, request: DeleteGameRequest) -> Result<(), Error> {
        self.repository.delete(request.id).await?;

        Ok(())
    }
}