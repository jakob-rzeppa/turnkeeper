use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{get_pos, is_token}, parsable::Parsable}, domain::game::value_objects::data::Value};

impl Parsable for Value {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::IntLiteral(_))
            || is_token!(ts, TokenVariant::FloatLiteral(_))
            || is_token!(ts, TokenVariant::StringLiteral(_))
            || is_token!(ts, TokenVariant::BoolLiteral(_))
    }

    fn parse(ts: &mut TokenStream, _source_code: &str) -> Result<Self, ParsingError> {
        match ts.next() {
            Some(token) => {
                let value = match &token.variant {
                    TokenVariant::IntLiteral(val) => Value::Int(val.clone()),
                    TokenVariant::FloatLiteral(val) => Value::Float(val.clone()),
                    TokenVariant::StringLiteral(val) => Value::String(val.clone()),
                    TokenVariant::BoolLiteral(val) => Value::Bool(val.clone()),
                    _ => {
                        return Err(ParsingError::UnexpectedToken {
                            expected: "a literal".to_string(),
                            found: token.variant.clone(),
                            pos: token.pos,
                        });
                    }
                };

                Ok(value)
            }
            None => Err(ParsingError::UnexpectedEOF {
                expected: "Expected a literal".to_string(),
            }),
        }
    }
}