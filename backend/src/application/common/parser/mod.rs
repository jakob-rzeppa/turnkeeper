
use crate::{application::common::parser::{error::ParsingError, lexer::{Lexer, token_stream::TokenStream}, parsable::Parsable}, domain::game::entities::weak::{action::Action, page::Page, stat::{GameStat, PlayerStat}}};

pub mod error;
pub mod lexer;
pub mod macros;
pub mod parsable;

pub mod parsables {
    pub mod atoms {
        pub mod variable_type;
    }

    pub mod roots {
        pub mod stat;
        pub mod player_stat;
        pub mod action;
    }
}

pub struct GameParsingResult {
    pub game_stats: Vec<GameStat>,
    pub player_stats: Vec<PlayerStat>,
    pub actions: Vec<Action>,
    pub pages: Vec<Page>,
}

pub struct GameParser {
    lexer: Lexer,
}

impl GameParser {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(),
        }
    }
}

#[mockall::automock]
pub trait GameParserContract: Send + Sync + 'static {
    fn parse_game(&self, source_code: &str) -> Result<GameParsingResult, ParsingError>;
}

impl GameParserContract for GameParser {
    fn parse_game(&self, source_code: &str) -> Result<GameParsingResult, ParsingError> {
        let tokens = self.lexer.tokenize(source_code)?;

        let mut token_stream = TokenStream::new(tokens);

        let mut game_stats = Vec::new();
        let mut player_stats = Vec::new();
        let mut actions = Vec::new();

        while token_stream.peek().is_some() {
            if PlayerStat::is_next(&token_stream) {
                player_stats.push(PlayerStat::parse(&mut token_stream, source_code)?);
            } else if GameStat::is_next(&token_stream) {
                game_stats.push(GameStat::parse(&mut token_stream, source_code)?);
            } else if Action::is_next(&token_stream) {
                actions.push(Action::parse(&mut token_stream, source_code)?);
            } else {
                return Err(ParsingError::UnexpectedToken {
                    expected: "PlayerStat or GameStat".to_string(),
                    found: token_stream.peek().unwrap().variant.clone(),
                    pos: token_stream.peek().unwrap().pos,
                });
            }
        }

        Ok(GameParsingResult {
            game_stats,
            player_stats,
            actions,
            pages: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::game::value_objects::{
        data::{VariableType, VariableValue},
        parameter::Parameter,
        visibility::{ActionVisibility, GameStatVisibility, PlayerStatVisibility},
    };

    use super::*;

    #[test]
    fn test_parse_game() {
        let parser = GameParser::new();

        let source_code = r#"protected pstat health: float = 10.0;
public stat score = 0;

public action heal(amount: float) {
    health = health + amount;
}"#;

        let result = parser.parse_game(source_code).unwrap();

        assert_eq!(result.player_stats.len(), 1);
        let health_stat = &result.player_stats[0];
        assert_eq!(health_stat.name(), "health");
        assert_eq!(health_stat.default(), &VariableValue::Float(10.0));
        assert_eq!(health_stat.visibility(), &PlayerStatVisibility::Protected);

        assert_eq!(result.game_stats.len(), 1);
        let score_stat = &result.game_stats[0];
        assert_eq!(score_stat.name(), "score");
        assert_eq!(score_stat.default(), &VariableValue::Int(0));
        assert_eq!(score_stat.visibility(), &GameStatVisibility::Public);

        assert_eq!(result.actions.len(), 1);
        let heal_action = &result.actions[0];
        assert_eq!(heal_action.name(), "heal");
        assert_eq!(
            heal_action.parameters(),
            &vec![Parameter::new("amount".to_string(), VariableType::Float)]
        );
        assert_eq!(heal_action.execution_triggers().len(), 0);
        assert_eq!(heal_action.visibility(), &ActionVisibility::Public);
        assert_eq!(
            heal_action.source_code(),
            r#"public action heal(amount: float) {
    health = health + amount;
}"#
            .to_string()
        );
    }
}
