use crate::domain::common::{identifier::Identifier, position::Position};

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    id: Identifier,
    name: String,

    source_code: String,
    pos: Position,
}

impl Page {
    pub fn new(name: String, source_code: String, pos: Position) -> Self {
        Self {
            id: Identifier::new(),
            name,
            source_code,
            pos,
        }
    }

    pub fn new_raw(id: Identifier, name: String, source_code: String, pos: Position) -> Self {
        Self {
            id,
            name,
            source_code,
            pos,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }
}
