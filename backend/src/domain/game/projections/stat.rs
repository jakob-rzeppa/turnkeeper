use crate::domain::{
    common::position::Position,
    game::value_objects::{
        data::{Datatype, Value},
        visibility::{GameStatVisibility, PlayerStatVisibility},
    },
};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GameStatMetadataProjection {
    pub name: String,
    pub datatype: Datatype,

    pub default: Value,
    pub visibility: GameStatVisibility,

    pub pos: Position,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PlayerStatMetadataProjection {
    pub name: String,
    pub datatype: Datatype,

    pub default: Value,
    pub visibility: PlayerStatVisibility,

    pub pos: Position,
}
