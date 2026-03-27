use crate::application::game::plugin::{
    common::Position,
    lexer::token::{Token, TokenVariant},
    parser::abstract_syntax_tree::statement::Statement,
    runtime::RuntimeEnvironment,
};

#[macro_use]
mod macros;

pub mod atom;
pub mod expression;
pub mod statement;

#[derive(Debug)]
pub enum ParsingError {
    SyntaxError {
        message: String,
        pos: Position,
    },
    UnexpectedToken {
        expected: String,
        found: TokenVariant,
        pos: Position,
    },
    UnexpectedEOF {
        expected: String,
    },
}

pub enum EvaluationError {}

pub trait Parsable
where
    Self: Sized,
{
    fn is_next(ts: &TokenStream) -> bool;

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError>;
}

pub trait Positioned {
    fn position(&self) -> Position;
}

pub trait Evaluable {
    fn evaluate(&self, runtime: &mut RuntimeEnvironment) -> Result<(), EvaluationError>;
}

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    fn next(&mut self) -> Option<&Token> {
        self.tokens.get(self.index).map(|token| {
            self.index += 1;
            token
        })
    }
}

pub struct Root {
    statements: Vec<Statement>,
}
