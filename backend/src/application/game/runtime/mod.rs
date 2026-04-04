use crate::{
    application::game::commands::GameCommand,
    domain::game::{
        entities::game::Game,
        error::GameError,
        projections::{
            game::{GameProjection, user::PlayerGameProjection},
            game_metadata::GameMetadata,
        },
        value_objects::id::Id,
    },
};

pub struct GameRuntime {
    game: Game,
}

impl GameRuntime {
    pub fn new(metadata: GameMetadata) -> Self {
        Self {
            game: Game::new(metadata.id, metadata.name, metadata.gm_user_id),
        }
    }

    /// Dispatches a [`GameCommand`] to the appropriate handler method.
    pub fn handle_command(&mut self, command: GameCommand) -> Result<(), GameError> {
        println!("Handling command: {:?}", command);
        match command {
            GameCommand::NextTurn => Ok(self.game.next_turn()),
            GameCommand::PreviousTurn => Ok(self.game.prev_turn()),
            GameCommand::SkipTurnToPlayer { player_id } => self.game.skip_turn_to_player(player_id),
            GameCommand::SetNotes(notes) => Ok(self.game.set_notes(notes)),
            GameCommand::SetHiddenNotes(hidden_notes) => {
                Ok(self.game.set_hidden_notes(hidden_notes))
            }
            GameCommand::AddPlayer { player_id } => self.game.add_player(player_id),
            GameCommand::AddStatToPlayer {
                player_id,
                stat_id,
                stat_key,
                stat_type,
                stat_value,
            } => self
                .game
                .add_stat_to_player(player_id, stat_id, stat_key, stat_type, stat_value),
            GameCommand::ChangeStatOfPlayer {
                player_id,
                stat_id,
                stat_type,
                stat_value,
            } => self
                .game
                .change_stat_of_player(player_id, stat_id, stat_type, stat_value),
            GameCommand::RemoveStatFromPlayer { player_id, stat_id } => {
                self.game.remove_stat_from_player(player_id, stat_id)
            }
            GameCommand::AddTradable {
                tradable_id,
                name,
                initial_value,
            } => self.game.add_tradable(tradable_id, name, initial_value),
            GameCommand::RemoveTradable { tradable_id } => self.game.remove_tradable(tradable_id),
            GameCommand::ChangePlayerTradableValue {
                player_id,
                tradable_id,
                new_value,
            } => self
                .game
                .change_player_tradable_value(player_id, tradable_id, new_value),
            GameCommand::SendTradable {
                from_id,
                to_id,
                tradable_id,
                amount,
            } => self.game.send_tradable(from_id, to_id, tradable_id, amount),
            GameCommand::AttachUserToPlayer { user_id, player_id } => {
                self.game.attach_user_to_player(user_id, player_id)
            }
            GameCommand::DetachUserFromPlayer { player_id } => {
                self.game.detach_user_from_player(player_id)
            }
            GameCommand::ChangePlayerOrder(ids_in_order) => {
                self.game.change_player_order(ids_in_order)
            }
            GameCommand::Debug(msg) => {
                println!("Debug command with message: {}", msg);
                Ok(())
            }
        }
    }

    pub fn get_id(&self) -> Id {
        *self.game.id()
    }

    pub fn get_gm_user_id(&self) -> Id {
        *self.game.gm_user_id()
    }

    pub fn get_user_ids(&self) -> Vec<Id> {
        self.game
            .players()
            .iter()
            .filter_map(|p| p.user_id())
            .collect()
    }

    pub fn get_gm_game_projection(&self) -> GameProjection {
        GameProjection::from(&self.game)
    }

    pub fn get_player_game_projection(&self, user_id: &Id) -> PlayerGameProjection {
        PlayerGameProjection::for_player_user_id(&self.game, user_id)
    }
}
