use crate::domain::{
    common::identifier::Identifier,
    game::value_objects::{
        stat_value::StatValue,
        stat_visibility::{GameStatVisibility, PlayerStatVisibility},
    },
};

pub struct GameStat {
    id: Identifier,

    name: String,
    value: StatValue,

    visibility: GameStatVisibility,
}

pub struct PlayerStat {
    id: Identifier,

    name: String,
    value: StatValue,

    visibility: PlayerStatVisibility,
}
