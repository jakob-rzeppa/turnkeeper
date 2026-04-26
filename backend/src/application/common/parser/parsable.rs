use crate::application::common::parser::{error::ParsingError, lexer::token_stream::TokenStream};

pub trait Parsable<Token>: Sized {
    fn is_next(ts: &TokenStream<Token>) -> bool;

    fn parse(ts: &mut TokenStream<Token>) -> Result<Self, ParsingError>;
}
