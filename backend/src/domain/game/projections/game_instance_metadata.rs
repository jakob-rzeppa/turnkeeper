use crate::domain::common::{date_time::DateTime, identifier::Identifier};

pub struct GameInstanceMetadataProjection {
    pub id: Identifier,
    pub name: String,

    pub game_id: Identifier,

    pub player_count: usize,
    pub current_round: u32,

    pub gm_user_id: Identifier,

    pub created_at: DateTime,
    pub last_played_at: DateTime,
}
