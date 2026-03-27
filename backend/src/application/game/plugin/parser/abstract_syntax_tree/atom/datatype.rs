use crate::application::game::plugin::{
    lexer::token::TokenVariant,
    parser::abstract_syntax_tree::{Parsable, ParsingError, TokenStream},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Datatype {
    Integer,
    Float,
    String,
    Boolean,
}

impl Parsable for Datatype {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::IntType)
            || is_token!(ts, TokenVariant::FloatType)
            || is_token!(ts, TokenVariant::StringType)
            || is_token!(ts, TokenVariant::BoolType)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let token = match ts.next() {
            Some(token) => token,
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected datatype".to_string(),
                });
            }
        };

        match &token.variant {
            TokenVariant::IntType => Ok(Datatype::Integer),
            TokenVariant::FloatType => Ok(Datatype::Float),
            TokenVariant::StringType => Ok(Datatype::String),
            TokenVariant::BoolType => Ok(Datatype::Boolean),
            _ => Err(ParsingError::UnexpectedToken {
                expected: "Expected datatype".to_string(),
                found: token.variant.clone(),
                pos: token.pos,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datatype_integer_parsing() {
        let mut ts = test_token_stream!(TokenVariant::IntType);

        assert!(Datatype::is_next(&ts));
        let datatype = Datatype::parse(&mut ts).unwrap();
        assert_eq!(datatype, Datatype::Integer);
    }

    #[test]
    fn test_datatype_float_parsing() {
        let mut ts = test_token_stream!(TokenVariant::FloatType);

        assert!(Datatype::is_next(&ts));
        let datatype = Datatype::parse(&mut ts).unwrap();
        assert_eq!(datatype, Datatype::Float);
    }

    #[test]
    fn test_datatype_string_parsing() {
        let mut ts = test_token_stream!(TokenVariant::StringType);

        assert!(Datatype::is_next(&ts));
        let datatype = Datatype::parse(&mut ts).unwrap();
        assert_eq!(datatype, Datatype::String);
    }

    #[test]
    fn test_datatype_boolean_parsing() {
        let mut ts = test_token_stream!(TokenVariant::BoolType);

        assert!(Datatype::is_next(&ts));
        let datatype = Datatype::parse(&mut ts).unwrap();
        assert_eq!(datatype, Datatype::Boolean);
    }

    #[test]
    fn test_datatype_parsing_error_on_non_datatype() {
        let mut ts = test_token_stream!(TokenVariant::Identifier("notAType".to_string()));

        assert!(!Datatype::is_next(&ts));
        let result = Datatype::parse(&mut ts);
        assert!(result.is_err());
    }
}
