use uuid::Uuid;
use crate::domain::entity::player::Player;

/// The representation of the game
///
/// # Creation
///
/// - For a new Game use `Game::new(id: Uuid)`.
/// - When instantiating an existing Game using `Game::builder()` is recommended.
/// 
/// # Invalid States
/// 
/// - Two Players have the same ID
/// - current_player_index is greater than length of players - 1
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