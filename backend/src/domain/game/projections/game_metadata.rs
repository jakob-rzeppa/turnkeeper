use crate::domain::common::{date_time::DateTime, identifier::Id};

#[derive(Debug, Clone)]
pub struct GameMetadataProjection {
    pub id: Id,
    pub name: String,
    pub description: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
