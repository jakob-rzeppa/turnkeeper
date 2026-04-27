use crate::application::common::parser::{error::ParsingError, lexer::token_stream::TokenStream};

pub trait Parsable: Sized {
    fn is_next(ts: &TokenStream) -> bool;

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError>;
}
