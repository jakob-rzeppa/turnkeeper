use crate::domain::game::entities::player::Player;
use crate::domain::game::entities::tradable::Tradable;
use crate::domain::game::error::{GameError};
use crate::domain::game::events::GameEvent;
use crate::domain::game::value_objects::id::Id;

mod player_events;
mod notes_events;
mod player_stat_events;
mod tradables_events;
mod turn_events;

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

    /// Dispatches a [`GameEvent`] to the appropriate handler method.
    pub fn handle_event(&mut self, event: GameEvent) -> Result<(), GameError> {
        println!("Handling event: {:?}", event);
        match event {
            GameEvent::NextTurn => Ok(self.next_turn()),
            GameEvent::PreviousTurn => Ok(self.prev_turn()),
            GameEvent::SkipTurnToPlayer { player_id } => self.skip_turn_to_player(player_id),
            GameEvent::SetNotes(notes) => Ok(self.set_notes(notes)),
            GameEvent::SetHiddenNotes(hidden_notes) => Ok(self.set_hidden_notes(hidden_notes)),
            GameEvent::AddPlayer { player_id } => self.add_player(player_id),
            GameEvent::AddStatToPlayer { player_id, stat_id, stat_key, stat_type, stat_value } => self.add_stat_to_player(
                player_id,
                stat_id,
                stat_key,
                stat_type,
                stat_value
            ),
            GameEvent::ChangeStatOfPlayer { player_id, stat_id, stat_type, stat_value } => self.change_stat_of_player(
                player_id,
                stat_id,
                stat_type,
                stat_value,
            ),
            GameEvent::RemoveStatFromPlayer { player_id, stat_id } => self.remove_stat_from_player(player_id, stat_id),
            GameEvent::AddTradable { tradable_id, name, initial_value } => {
                self.add_tradable(
                    tradable_id,
                    name,
                    initial_value)
            },
            GameEvent::RemoveTradable { tradable_id } => self.remove_tradable(tradable_id),
            GameEvent::ChangePlayerTradableValue { player_id, tradable_id, new_value } => {
                self.change_player_tradable_value(
                    player_id,
                    tradable_id,
                    new_value)
            },
            GameEvent::SendTradable {from_id, to_id, tradable_id, amount } => {
                self.send_tradable(
                    from_id,
                    to_id,
                    tradable_id,
                    amount)
            },
            GameEvent::AttachUserToPlayer { user_id, player_id } => self.attach_user_to_player(
                user_id,
                player_id,
            ),
            GameEvent::DetachUserFromPlayer { player_id } => self.detach_user_from_player(
                player_id,
            ),
            GameEvent::ChangePlayerOrder(ids_in_order) => {
                self.change_player_order(ids_in_order)
            },
            GameEvent::Debug(msg) => {
                println!("Debug event with message: {}", msg);
                Ok(())
            }
        }
    }
}
