use crate::application::plugin::{
    lexer::token::TokenVariant,
    parser::abstract_syntax_tree::{Parsable, ParsingError, TokenStream},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Parsable for Identifier {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Identifier(_))
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let token = match ts.next() {
            Some(token) => token,
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "identifier".to_string(),
                });
            }
        };

        match &token.variant {
            TokenVariant::Identifier(name) => Ok(Identifier { name: name.clone() }),
            _ => Err(ParsingError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: token.variant.clone(),
                pos: token.pos,
            }),
        }
    }
}

#[cfg(test)]
impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_parsing() {
        let mut ts = test_token_stream!(TokenVariant::Identifier("myVariable".to_string()));

        assert!(Identifier::is_next(&ts));
        let identifier = Identifier::parse(&mut ts).unwrap();
        assert_eq!(identifier, Identifier::new("myVariable"));
    }

    #[test]
    fn test_identifier_parsing_single_char() {
        let mut ts = test_token_stream!(TokenVariant::Identifier("x".to_string()));

        assert!(Identifier::is_next(&ts));
        let identifier = Identifier::parse(&mut ts).unwrap();
        assert_eq!(identifier, Identifier::new("x"));
    }

    #[test]
    fn test_identifier_parsing_with_underscore() {
        let mut ts = test_token_stream!(TokenVariant::Identifier("my_variable".to_string()));

        assert!(Identifier::is_next(&ts));
        let identifier = Identifier::parse(&mut ts).unwrap();
        assert_eq!(identifier, Identifier::new("my_variable"));
    }

    #[test]
    fn test_identifier_parsing_error_on_non_identifier() {
        let mut ts = test_token_stream!(TokenVariant::IntLiteral(42));

        assert!(!Identifier::is_next(&ts));
        let result = Identifier::parse(&mut ts);
        assert!(result.is_err());
    }
}
