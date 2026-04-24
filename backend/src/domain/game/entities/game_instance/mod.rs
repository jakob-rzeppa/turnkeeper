use std::collections::HashMap;

use crate::domain::{
    common::identifier::Identifier,
    game::entities::{
        action::Action,
        game::Game,
        log::Log,
        page::Page,
        player::Player,
        stat::{GameStat, PlayerStat},
    },
};

pub mod player_management;
pub mod projection_management;
pub mod state_management;

pub struct GameInstance {
    id: Identifier,
    name: String,

    current_player_index: usize,
    round: u32,
    game_stats: Vec<GameStat>,
    player_stats: Vec<PlayerStat>,
    actions: Vec<Action>,
    pages: Vec<Page>,

    players: Vec<Player>,

    log: Log,

    source: Game,
}

impl GameInstance {
    pub fn new(
        name: String,
        game_stats: Vec<GameStat>,
        player_stats: Vec<PlayerStat>,
        actions: Vec<Action>,
        pages: Vec<Page>,
        source: Game,
    ) -> Self {
        Self {
            id: Identifier::new(),
            name,
            log: Log::new(),
            current_player_index: 0,
            round: 1,
            game_stats,
            player_stats,
            actions,
            pages,
            players: Vec::new(),
            source,
        }
    }

    pub fn new_raw(
        id: Identifier,
        name: String,
        current_player_index: usize,
        round: u32,
        game_stats: Vec<GameStat>,
        player_stats: Vec<PlayerStat>,
        actions: Vec<Action>,
        pages: Vec<Page>,
        players: Vec<Player>,
        log: Log,
        source: Game,
    ) -> Self {
        Self {
            id,
            name,
            current_player_index,
            round,
            game_stats,
            player_stats,
            actions,
            pages,
            players,
            log,
            source,
        }
    }
}
