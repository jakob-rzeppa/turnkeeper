use crate::domain::{common::position::Position, game::value_objects::visibility::PageVisibility};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PageMetadataProjection {
    pub name: String,

    pub visibility: PageVisibility,

    pub source_code: String,
    pub pos: Position,
}
