use uuid::Uuid;
use crate::domain::game::events::GameEvent;

pub struct GameOverviewDto {
    id: Uuid,
    name: String,
}

#[derive(Debug)]
pub enum ConnectionMessageDto {
    Event(GameEvent),
    Close,
}