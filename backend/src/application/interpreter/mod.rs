use crate::application::game::root_parser::error::GameParsingError;

pub mod error;
pub mod parser;

pub trait Parsable: Sized {
    fn is_next(ts: &TokenStream) -> bool;

    fn parse(ts: &mut TokenStream) -> Result<Self, GameParsingError>;
}

pub struct TokenStream {
    tokens: Vec<()>,
    index: usize,
}
