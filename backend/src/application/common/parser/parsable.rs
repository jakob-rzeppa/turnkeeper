use crate::application::common::parser::token_stream::TokenStream;

pub trait Parsable<Token>: Sized {
    type Error;

    fn is_next(ts: &TokenStream<Token>) -> bool;

    fn parse(ts: &mut TokenStream<Token>) -> Result<Self, Self::Error>;
}
