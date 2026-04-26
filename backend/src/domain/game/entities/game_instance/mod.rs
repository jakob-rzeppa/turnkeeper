use crate::domain::{
    common::{date_time::DateTime, identifier::Identifier},
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
pub mod stats_management;

#[derive(Debug, Clone, PartialEq)]
pub struct GameInstance {
    id: Identifier,
    name: String,

    current_player_index: usize,
    round: u32,
    game_stats: Vec<GameStat>,
    player_stats: Vec<PlayerStat>,
    actions: Vec<Action>,
    pages: Vec<Page>,

    gm_user_id: Identifier,
    players: Vec<Player>,

    log: Log,

    source_game: Game,

    created_at: DateTime,
    last_played_at: DateTime,
}

impl GameInstance {
    pub fn new(
        name: String,
        gm_user_id: Identifier,
        game_stats: Vec<GameStat>,
        player_stats: Vec<PlayerStat>,
        actions: Vec<Action>,
        pages: Vec<Page>,
        source_game: Game,
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
            source_game,
            gm_user_id,
            created_at: DateTime::now(),
            last_played_at: DateTime::now(),
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
        source_game: Game,
        gm_user_id: Identifier,
        created_at: DateTime,
        last_played_at: DateTime,
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
            source_game,
            gm_user_id,
            created_at,
            last_played_at,
        }
    }

    pub fn id(&self) -> &Identifier {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn current_player_index(&self) -> usize {
        self.current_player_index
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn game_stats(&self) -> &Vec<GameStat> {
        &self.game_stats
    }

    pub fn player_stats(&self) -> &Vec<PlayerStat> {
        &self.player_stats
    }

    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }

    pub fn pages(&self) -> &Vec<Page> {
        &self.pages
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn source_game(&self) -> &Game {
        &self.source_game
    }

    pub fn gm_user_id(&self) -> &Identifier {
        &self.gm_user_id
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }

    pub fn last_played_at(&self) -> &DateTime {
        &self.last_played_at
    }
}
