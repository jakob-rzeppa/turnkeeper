use crate::{
    application::common::parser::lexer::token::TokenVariant, domain::common::position::Position,
};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParsingError {
    #[error("Invalid token at position {pos}: {message}")]
    InvalidToken { pos: Position, message: String },
    #[error("Syntax error at position {pos}: {message}")]
    SyntaxError { message: String, pos: Position },
    #[error("Unexpected token at position {pos}: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: TokenVariant,
        pos: Position,
    },
    #[error("Unexpected end of file: expected {expected}")]
    UnexpectedEOF { expected: String },
}
