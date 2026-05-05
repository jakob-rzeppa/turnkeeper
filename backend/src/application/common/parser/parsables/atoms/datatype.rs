use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{token::TokenVariant, token_stream::TokenStream},
        macros::is_token,
        parsable::Parsable,
    },
    domain::game::value_objects::data::Datatype,
};

impl Parsable for Datatype {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(
            ts,
            TokenVariant::IntType
                | TokenVariant::FloatType
                | TokenVariant::StringType
                | TokenVariant::BoolType
        )
    }

    fn parse(ts: &mut TokenStream, _source_code: &str) -> Result<Self, ParsingError> {
        let token = ts.next().ok_or_else(|| ParsingError::UnexpectedEOF {
            expected: "Expected variable type (int, float, string, bool)".to_string(),
        })?;

        match token.variant {
            TokenVariant::IntType => Ok(Datatype::Int),
            TokenVariant::FloatType => Ok(Datatype::Float),
            TokenVariant::StringType => Ok(Datatype::String),
            TokenVariant::BoolType => Ok(Datatype::Bool),
            _ => Err(ParsingError::UnexpectedToken {
                expected: "Expected variable type (int, float, string, bool)".to_string(),
                found: token.variant.clone(),
                pos: token.pos,
            }),
        }
    }
}
