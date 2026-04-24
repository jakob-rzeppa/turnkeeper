use fnmock::derive::{fake_function, mock_function};

use crate::{
    application::interpreter::error::GameParsingError,
    domain::game::entities::{
        action::Action,
        page::Page,
        stat::{GameStat, PlayerStat},
    },
};

pub mod error;

pub struct Position {
    line: usize,
    column: usize,
}

pub trait Parsable: Sized {
    fn is_next(ts: &TokenStream) -> bool;

    fn parse(ts: &mut TokenStream) -> Result<Self, GameParsingError>;
}

pub trait Positioned {
    fn position(&self) -> Position;
}

pub struct TokenStream {
    tokens: Vec<()>,
    index: usize,
}

pub struct GameParsingResult {
    pub game_stats: Vec<GameStat>,
    pub player_stats: Vec<PlayerStat>,
    pub actions: Vec<Action>,
    pub pages: Vec<Page>,
}

#[fake_function]
pub fn parse_game(source_code: &str) -> Result<GameParsingResult, GameParsingError> {
    unimplemented!()
}
