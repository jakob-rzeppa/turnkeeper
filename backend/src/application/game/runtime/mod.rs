use crate::domain::game::{commands::GameCommand, entities::game::Game, error::GameError, projections::{gm_game_info::GmGameInfo, user_game_info::UserGameInfo}, value_objects::id::Id};



pub struct GameRuntime {
    game: Game,
}

impl GameRuntime {
    pub fn new(id: Id, name: String) -> Self {
        Self { game: Game::new(id, name) }
    }

    pub fn handle_command(&mut self, command: GameCommand) -> Result<(), GameError> {
        self.game.handle_command(command)
    }

    pub fn get_id(&self) -> Id {
        *self.game.id()
    }

    pub fn get_gm_game_projection(&self) -> GmGameInfo {
        GmGameInfo::from(&self.game)
    }

    pub fn get_user_game_projection(&self, user_id: &Id) -> UserGameInfo {
        UserGameInfo::for_user(&self.game, user_id)
    }
}