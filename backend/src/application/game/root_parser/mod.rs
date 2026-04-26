use crate::{
    application::{
        common::parser::{parsable::Parsable, scanner::Scanner, token_stream::TokenStream},
        game::root_parser::error::GameParsingError,
    },
    domain::game::entities::{
        action::Action,
        page::Page,
        stat::{GameStat, PlayerStat},
    },
};

pub mod error;
mod parsables {
    pub mod player_stat;
    pub mod stat;
}
mod token;

pub struct GameParsingResult {
    pub game_stats: Vec<GameStat>,
    pub player_stats: Vec<PlayerStat>,
    pub actions: Vec<Action>,
    pub pages: Vec<Page>,
}

pub struct GameRootParser {
    scanner: Scanner,
}

impl GameRootParser {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(),
        }
    }
}

#[mockall::automock]
pub trait GameRootParserContract {
    fn parse_game(&self, source_code: &str) -> Result<GameParsingResult, GameParsingError>;
}

impl GameRootParserContract for GameRootParser {
    fn parse_game(&self, source_code: &str) -> Result<GameParsingResult, GameParsingError> {
        let lexemes = self.scanner.scan_source_code(source_code);

        let tokens = lexemes
            .into_iter()
            .map(|lexeme| token::Token::try_from(lexeme))
            .collect::<Result<Vec<_>, _>>()?;

        let mut token_stream = TokenStream::new(tokens);

        let mut game_stats = Vec::new();
        let mut player_stats = Vec::new();

        while token_stream.peek().is_some() {
            if PlayerStat::is_next(&token_stream) {
                player_stats.push(PlayerStat::parse(&mut token_stream)?);
            } else if GameStat::is_next(&token_stream) {
                game_stats.push(GameStat::parse(&mut token_stream)?);
            } else {
                return Err(GameParsingError::UnexpectedToken {
                    expected: "PlayerStat or GameStat".to_string(),
                    found: token_stream.peek().unwrap().variant.clone(),
                    pos: token_stream.peek().unwrap().pos,
                });
            }
        }

        Ok(GameParsingResult {
            game_stats,
            player_stats,
            actions: Vec::new(),
            pages: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::game::value_objects::{
        stat_value::StatValue,
        stat_visibility::{GameStatVisibility, PlayerStatVisibility},
    };

    use super::*;

    #[test]
    fn test_parse_game() {
        let parser = GameRootParser::new();

        let source_code = r#"
            protected pstat health: float = 10.0;
            public stat score = 0;
        "#;

        let result = parser.parse_game(source_code);
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.player_stats.len(), 1);
        let health_stat = &result.player_stats[0];
        assert_eq!(health_stat.name(), "health");
        assert_eq!(health_stat.default(), &StatValue::Float(10.0));
        assert_eq!(health_stat.visibility(), &PlayerStatVisibility::Protected);

        assert_eq!(result.game_stats.len(), 1);
        let score_stat = &result.game_stats[0];
        assert_eq!(score_stat.name(), "score");
        assert_eq!(score_stat.default(), &StatValue::Int(0));
        assert_eq!(score_stat.visibility(), &GameStatVisibility::Public);
    }
}
