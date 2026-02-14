use uuid::Uuid;
use crate::domain::game::entities::game::Game;
use crate::domain::game::events::GameEvent;

pub struct GameEventHandler {
    game: Game,
}

impl GameEventHandler {
    pub fn new(game_id: Uuid, game_name: String) -> Self {
        Self { game: Game::new(game_id, game_name) }
    }

    pub fn handle(&mut self, event: GameEvent) {}
}