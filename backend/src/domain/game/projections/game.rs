use crate::domain::common::{date_time::DateTime, identifier::Identifier};

pub struct GameProjection {
    pub id: Identifier,
    pub name: String,
    pub description: String,

    pub source_code: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
