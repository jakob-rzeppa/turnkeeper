use crate::domain::game::entities::player::Player;
use crate::domain::game::entities::tradable::Tradable;
use crate::domain::game::error::{GameError};
use crate::domain::game::commands::GameCommand;
use crate::domain::game::value_objects::id::Id;

mod player_commands;
mod notes_commands;
mod player_stat_commands;
mod tradables_commands;
mod turn_commands;

/// The aggregate root representing a game.
///
/// # Invariants
///
/// - No two players have the same ID
/// - `current_player_index` does not exceed `players.len() - 1`
/// - All players are represented in all tradable values (if a player is added, they should be added to all tradables with a default value, and if a player is removed, they should be removed from all tradables)
#[derive(Debug, PartialEq)]
pub struct Game {
    id: Id,
    name: String,

    players: Vec<Player>,
    tradables: Vec<Tradable>,

    round_number: u32,
    current_player_index: usize,

    notes: String,
    hidden_notes: String,
}

impl Game {
    pub fn new(id: Id, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            tradables: Vec::new(),
            round_number: 0,
            current_player_index: 0,
            notes: String::new(),
            hidden_notes: String::new(),
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn tradables(&self) -> &[Tradable] {
        &self.tradables
    }

    pub fn round_number(&self) -> u32 {
        self.round_number
    }

    pub fn current_player_index(&self) -> usize {
        self.current_player_index
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn hidden_notes(&self) -> &str {
        &self.hidden_notes
    }

    /// Dispatches a [`GameCommand`] to the appropriate handler method.
    pub fn handle_command(&mut self, command: GameCommand) -> Result<(), GameError> {
        println!("Handling command: {:?}", command);
        match command {
            GameCommand::NextTurn => Ok(self.next_turn()),
            GameCommand::PreviousTurn => Ok(self.prev_turn()),
            GameCommand::SkipTurnToPlayer { player_id } => self.skip_turn_to_player(player_id),
            GameCommand::SetNotes(notes) => Ok(self.set_notes(notes)),
            GameCommand::SetHiddenNotes(hidden_notes) => Ok(self.set_hidden_notes(hidden_notes)),
            GameCommand::AddPlayer { player_id } => self.add_player(player_id),
            GameCommand::AddStatToPlayer { player_id, stat_id, stat_key, stat_type, stat_value } => self.add_stat_to_player(
                player_id,
                stat_id,
                stat_key,
                stat_type,
                stat_value
            ),
            GameCommand::ChangeStatOfPlayer { player_id, stat_id, stat_type, stat_value } => self.change_stat_of_player(
                player_id,
                stat_id,
                stat_type,
                stat_value,
            ),
            GameCommand::RemoveStatFromPlayer { player_id, stat_id } => self.remove_stat_from_player(player_id, stat_id),
            GameCommand::AddTradable { tradable_id, name, initial_value } => {
                self.add_tradable(
                    tradable_id,
                    name,
                    initial_value)
            },
            GameCommand::RemoveTradable { tradable_id } => self.remove_tradable(tradable_id),
            GameCommand::ChangePlayerTradableValue { player_id, tradable_id, new_value } => {
                self.change_player_tradable_value(
                    player_id,
                    tradable_id,
                    new_value)
            },
            GameCommand::SendTradable {from_id, to_id, tradable_id, amount } => {
                self.send_tradable(
                    from_id,
                    to_id,
                    tradable_id,
                    amount)
            },
            GameCommand::AttachUserToPlayer { user_id, player_id } => self.attach_user_to_player(
                user_id,
                player_id,
            ),
            GameCommand::DetachUserFromPlayer { player_id } => self.detach_user_from_player(
                player_id,
            ),
            GameCommand::ChangePlayerOrder(ids_in_order) => {
                self.change_player_order(ids_in_order)
            },
            GameCommand::Debug(msg) => {
                println!("Debug command with message: {}", msg);
                Ok(())
            }
        }
    }
}
