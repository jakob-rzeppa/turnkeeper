use crate::domain::common::identifier::Identifier;

pub struct Game {
    id: Identifier,

    name: String,
    description: String,

    source_code: String,
}

impl Game {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Identifier::new(),
            name,
            description,
            source_code: String::new(),
        }
    }
}
