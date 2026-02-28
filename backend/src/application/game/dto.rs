use crate::domain::game::events::GameEvent;

#[derive(Debug)]
pub enum ConnectionMessageDto {
    Event(GameEvent),
    Unknown,
    Close,
}