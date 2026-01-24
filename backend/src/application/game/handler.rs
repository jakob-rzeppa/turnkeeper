use uuid::Uuid;
use crate::error::ApplicationError;

pub trait GameHandler {
    fn create_game(&self, id: Uuid) -> Result<Uuid, ApplicationError>;
    fn delete_game(&self, id: Uuid) -> Result<(), ApplicationError>;
}