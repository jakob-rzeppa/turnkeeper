use crate::{
    application::common::parser::lexer::token::TokenVariant,
    domain::common::position::Position,
};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParsingError {
    #[error("Invalid token at position {pos}: {message}")] InvalidToken {
        pos: Position,
        message: String,
    },
    #[error("Syntax error at position {pos}: {message}")] SyntaxError {
        message: String,
        pos: Position,
    },
    #[error(
        "Unexpected token at position {pos}: expected {expected}, found {found}"
    )] UnexpectedToken {
        expected: String,
        found: TokenVariant,
        pos: Position,
    },
    #[error("Unexpected end of file: expected {expected}")] UnexpectedEOF {
        expected: String,
    },
    #[error("Duplicate player stat {name} at position {pos}")] DuplicatePlayerStat {
        name: String,
        pos: Position,
    },
    #[error("Duplicate game stat {name} at position {pos}")] DuplicateGameStat {
        name: String,
        pos: Position,
    },
    #[error("Duplicate action {name} at position {pos}")] DuplicateAction {
        name: String,
        pos: Position,
    },
}

impl ParsingError {
    pub fn projection(&self) -> ParsingErrorProjection {
        match self {
            ParsingError::InvalidToken { pos, message } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: pos.to_string(),
                },
            ParsingError::SyntaxError { message, pos } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: pos.to_string(),
                },
            ParsingError::UnexpectedToken { expected, found, pos } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: pos.to_string(),
                },
            ParsingError::UnexpectedEOF { expected } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: "EOF".to_string(),
                },
            ParsingError::DuplicatePlayerStat { name, pos } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: pos.to_string(),
                },
            ParsingError::DuplicateGameStat { name, pos } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: pos.to_string(),
                },
            ParsingError::DuplicateAction { name, pos } =>
                ParsingErrorProjection {
                    message: self.to_string(),
                    pos: pos.to_string(),
                },
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ParsingErrorProjection {
    pub message: String,
    pub pos: String, // (line, column) or EOF
}
