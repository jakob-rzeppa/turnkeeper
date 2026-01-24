use uuid::Uuid;
use crate::application::game::dto::GameOverviewDto;
use crate::error::ApplicationError;

/// The GameHandler is the central service handling all the games.
pub struct GameHandler {}

impl GameHandler {
    pub fn get_games_overview() -> Result<Vec<GameOverviewDto>, ApplicationError> {
        Err(ApplicationError::NotImplemented)
    }

    pub fn create_game() -> Result<Uuid, ApplicationError> {
        Err(ApplicationError::NotImplemented)
    }

    pub fn delete_game(id: Uuid) -> Result<(), ApplicationError> {
        Err(ApplicationError::NotImplemented)
    }
}