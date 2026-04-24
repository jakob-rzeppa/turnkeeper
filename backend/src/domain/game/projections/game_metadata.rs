use crate::domain::common::{date_time::DateTime, identifier::Identifier};

#[derive(Debug, Clone)]
pub struct GameMetadataProjection {
    pub id: Identifier,
    pub name: String,
    pub description: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
