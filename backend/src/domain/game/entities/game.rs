use crate::domain::{
    common::{date_time::DateTime, identifier::Identifier},
    game::projections::{game::GameProjection, game_metadata::GameMetadataProjection},
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime {
        &self.updated_at
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

    pub fn set_source_code(&mut self, source_code: String) {
        self.source_code = source_code;
        self.updated_at = DateTime::now();
    }

    pub fn get_metadata_projection(&self) -> GameMetadataProjection {
        GameMetadataProjection {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
