use uuid::Uuid;
use crate::domain::entity::player::Player;
use crate::domain::value_object::identity::Identity;

/// The representation of the game
///
/// # Creation
///
/// - For a new Game use `Game::new(id: Uuid)`.
/// - When instantiating a existing Game using `Game::builder()` is recommended.
pub struct Game {
    id: Identity,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,
}

impl Game {
    pub fn new(id: Identity) -> Self {
        Self {
            id,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
        }
    }
}