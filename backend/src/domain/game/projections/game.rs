use crate::domain::common::{date_time::DateTime, identifier::Id};

pub struct GameProjection {
    pub id: Id,
    pub name: String,
    pub description: String,

    pub source_code: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
