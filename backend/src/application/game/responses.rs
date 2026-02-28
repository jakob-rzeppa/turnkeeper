use crate::domain::game::projections::game_metadata::GameMetadata;

pub struct OverviewGameResponse {
    pub games_metadata: Vec<GameMetadata>,
}
