use crate::domain::common::{date_time::DateTime, identifier::Id};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GameInstanceMetadataProjection {
    pub id: Id,
    pub name: String,

    pub game_id: Id,

    pub player_count: usize,
    pub current_round: u32,

    pub gm_user_id: Id,

    pub created_at: DateTime,
    pub last_played_at: DateTime,
}
