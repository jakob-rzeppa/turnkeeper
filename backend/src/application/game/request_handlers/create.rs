use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::requests::CreateGameRequest;
use crate::domain::game::error::GameError;

pub struct CreateGameRequestHandler<GameRepository: GameRepositoryContract + 'static> {
    repository: GameRepository,
}

impl<GameRepository: GameRepositoryContract + 'static> CreateGameRequestHandler<GameRepository> {
    pub fn new(repository: GameRepository) -> Self { Self { repository } }

    pub async fn create_game(&self, request: CreateGameRequest) -> Result<Uuid, GameError> {
        let id = Uuid::new_v4();

        self.repository.create(id.clone(), request.name).await?;

        Ok(id)
    }
}