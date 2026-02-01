use uuid::Uuid;
use crate::domain::game::entities::player::Player;

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
    name: String,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,
}

impl Game {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}