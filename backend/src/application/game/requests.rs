use uuid::Uuid;
use crate::domain::game::projections::GameMetadata;

pub struct CreateGameRequest {
    pub name: String,
}

pub struct DeleteGameRequest {
    pub id: Uuid,
}

pub struct OverviewGameResponse {
    pub games_metadata: Vec<GameMetadata>,
}