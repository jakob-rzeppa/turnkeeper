use crate::domain::common::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct GameMetadataProjection {
    pub id: Identifier,
    pub name: String,
    pub description: String,

    pub created_at: String,
    pub updated_at: String,
}
