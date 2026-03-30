use crate::application::game::plugin::{
    common::Position, lexer::token::Token, parser::abstract_syntax_tree::error::ParsingError,
};

#[macro_use]
mod macros;

pub mod atom;
pub mod error;
pub mod expression;
pub mod root;
pub mod statement;

pub trait Parsable: Sized {
    fn is_next(ts: &TokenStream) -> bool;

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError>;
}

pub trait Positioned {
    fn position(&self) -> Position;
}

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.tokens.get(self.index).map(|token| {
            self.index += 1;
            token
        })
    }
}
