use uuid::Uuid;

pub struct GameMetadata {
    pub id: Uuid,
    pub name: String,
    pub number_of_players: usize,
    pub round_number: u32,
    pub current_player_index: usize,
}