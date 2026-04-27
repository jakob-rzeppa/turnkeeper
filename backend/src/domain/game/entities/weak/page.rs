use crate::domain::{common::position::Position, game::value_objects::visibility::PageVisibility};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Page {
    name: String,

    visibility: PageVisibility,

    source_code: String,
    pos: Position,
}

impl Page {
    pub fn new(
        name: String,
        visibility: PageVisibility,
        source_code: String,
        pos: Position,
    ) -> Self {
        Self {
            name,
            visibility,
            source_code,
            pos,
        }
    }

    pub fn new_raw(
        name: String,
        visibility: PageVisibility,
        source_code: String,
        pos: Position,
    ) -> Self {
        Self {
            name,
            visibility,
            source_code,
            pos,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn visibility(&self) -> &PageVisibility {
        &self.visibility
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }
}
