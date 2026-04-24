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
    game_stats: HashMap<Identifier, GameStat>,
    player_stats: HashMap<Identifier, PlayerStat>,
    actions: HashMap<Identifier, Action>,
    pages: HashMap<Identifier, Page>,

    players: Vec<Player>,

    log: Log,

    source: Game,
}

impl GameInstance {
    pub fn new(
        name: String,
        game_stats: HashMap<Identifier, GameStat>,
        player_stats: HashMap<Identifier, PlayerStat>,
        actions: HashMap<Identifier, Action>,
        pages: HashMap<Identifier, Page>,
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
}
