use uuid::Uuid;
use crate::domain::entity::player::Player;

/// The representation of the game
///
/// # Creation
///
/// - For a new Game use `Game::new(id: Uuid)`.
/// - When instantiating a existing Game using `Game::builder()` is recommended.
pub struct Game {
    id: Uuid,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,
}

impl Game {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
        }
    }
}