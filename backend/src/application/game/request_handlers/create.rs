use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::requests::CreateGameRequest;
use crate::domain::error::Error;
use crate::domain::game::entities::game::Game;

pub struct CreateRequestHandler<GameRepository: GameRepositoryContract + 'static> {
    repository: GameRepository,
}

impl<GameRepository: GameRepositoryContract + 'static> CreateRequestHandler<GameRepository> {
    pub fn new(repository: GameRepository) -> Self { Self { repository } }

    pub async fn create_game(&self, request: CreateGameRequest) -> Result<(), Error> {
        let game = Game::new(Uuid::new_v4(), request.name);

        self.repository.save(game).await?;

        Ok(())
    }
}