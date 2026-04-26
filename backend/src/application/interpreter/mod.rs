use crate::application::common::parser::error::ParsingError;

pub mod error;
pub mod parser;

pub trait Parsable: Sized {
    fn is_next(ts: &TokenStream) -> bool;

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError>;
}

pub struct TokenStream {
    tokens: Vec<()>,
    index: usize,
}
