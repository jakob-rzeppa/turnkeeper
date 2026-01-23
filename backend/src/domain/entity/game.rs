use uuid::Uuid;
use crate::domain::entity::player::Player;
use crate::domain::value_object::identifier::Identifier;

/// The representation of the game
/// 
/// # Creation
/// 
/// - For a new Game use `Game::new(id: Uuid)`.
/// - When instantiating a existing Game using `Game::builder()` is recommended.
pub struct Game {
    id: Identifier,

    players: Vec<Player>,

    round_number: u32,
    current_player_index: usize,
}

impl Game {
    pub fn new(id: Uuid) -> Self {
        let id = Identifier::new(id);
        
        Self {
            id,
            players: Vec::new(),
            round_number: 0,
            current_player_index: 0,
        }
    }
    
    pub fn builder() -> GameBuilder {
        GameBuilder::default()
    }
}

#[derive(Default)]
pub struct GameBuilder {
    id: Option<Identifier>,
    players: Vec<Player>,

    round_number: Option<u32>,
    current_player_index: Option<usize>,
}

impl GameBuilder {
}