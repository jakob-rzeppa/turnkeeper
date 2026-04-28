use crate::domain::{
    common::position::Position,
    game::value_objects::{
        data::{VariableType, VariableValue},
        visibility::{GameStatVisibility, PlayerStatVisibility},
    },
};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GameStatMetadataProjection {
    pub name: String,
    pub datatype: VariableType,

    pub default: VariableValue,
    pub visibility: GameStatVisibility,

    pub pos: Position,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PlayerStatMetadataProjection {
    pub name: String,
    pub datatype: VariableType,

    pub default: VariableValue,
    pub visibility: PlayerStatVisibility,

    pub pos: Position,
}
