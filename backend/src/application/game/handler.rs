use uuid::Uuid;
use crate::application::game::dto::GameOverviewDto;
use crate::domain::error::Error;

/// The GameHandler is the central service handling all the games.
pub struct GameHandler {}

impl GameHandler {
    pub fn get_games_overview() -> Result<Vec<GameOverviewDto>, Error> {
        Err(Error::NotImplemented)
    }

    pub fn create_game() -> Result<Uuid, Error> {
        Err(Error::NotImplemented)
    }

    pub fn delete_game(id: Uuid) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}