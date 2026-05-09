use crate::domain::game::projections::{action::ActionMetadataProjection, stat::{GameStatMetadataProjection, PlayerStatMetadataProjection}};

#[derive(Debug, Clone, serde::Serialize)]
pub struct GameDisplayTemplateProjection {
    pub stats: Vec<GameStatMetadataProjection>,
    pub player_stats: Vec<PlayerStatMetadataProjection>,
    pub actions: Vec<ActionMetadataProjection>,
}
