use crate::{
    application::common::parser::{
        error::ParsingError,
        lexer::{token::TokenVariant, token_stream::TokenStream},
        macros::is_token,
        parsable::Parsable,
    },
    domain::game::value_objects::data::VariableType,
};

impl Parsable for VariableType {
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
            TokenVariant::IntType => Ok(VariableType::Int),
            TokenVariant::FloatType => Ok(VariableType::Float),
            TokenVariant::StringType => Ok(VariableType::String),
            TokenVariant::BoolType => Ok(VariableType::Bool),
            _ => Err(ParsingError::UnexpectedToken {
                expected: "Expected variable type (int, float, string, bool)".to_string(),
                found: token.variant.clone(),
                pos: token.pos,
            }),
        }
    }
}
