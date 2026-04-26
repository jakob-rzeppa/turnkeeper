use crate::application::common::parser::{
    error::ParsingError,
    lexer::{scanner::scan_source_code, token::Token},
};

mod lexeme;
mod scanner;
pub mod token;
pub mod token_stream;

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&self, source_code: &str) -> Result<Vec<Token>, ParsingError> {
        let lexemes = scan_source_code(source_code);

        lexemes
            .into_iter()
            .map(|lexeme| lexeme.try_into())
            .collect::<Result<Vec<_>, _>>()
    }
}
