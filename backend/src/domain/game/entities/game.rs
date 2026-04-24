use crate::domain::{
    common::{date_time::DateTime, identifier::Identifier},
    game::projections::game::GameProjection,
};

pub struct Game {
    id: Identifier,

    name: String,
    description: String,

    source_code: String,

    created_at: DateTime,
    updated_at: DateTime,
}

impl Game {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Identifier::new(),
            name,
            description,
            source_code: String::new(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub fn new_raw(
        id: Identifier,
        name: String,
        description: String,
        source_code: String,
        created_at: DateTime,
        updated_at: DateTime,
    ) -> Self {
        Self {
            id,
            name,
            description,
            source_code,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    pub fn get_projection(&self) -> GameProjection {
        GameProjection {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            source_code: self.source_code.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
