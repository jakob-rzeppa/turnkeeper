use crate::domain::game::entities::player::Player;
use crate::domain::game::entities::tradable::Tradable;
use crate::domain::game::value_objects::id::Id;

mod notes_commands;
mod player_commands;
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
}
