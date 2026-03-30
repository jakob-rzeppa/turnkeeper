use crate::application::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::{
        abstract_syntax_tree::{Parsable, Positioned, TokenStream},
        error::ParsingError,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpressionAtom {
    value: LiteralValue,
    pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl LiteralExpressionAtom {
    pub fn value(&self) -> &LiteralValue {
        &self.value
    }
}

impl Parsable for LiteralExpressionAtom {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::IntLiteral(_))
            || is_token!(ts, TokenVariant::FloatLiteral(_))
            || is_token!(ts, TokenVariant::StringLiteral(_))
            || is_token!(ts, TokenVariant::BoolLiteral(_))
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        match ts.next() {
            Some(token) => {
                let value = match &token.variant {
                    TokenVariant::IntLiteral(val) => LiteralValue::Int(val.clone()),
                    TokenVariant::FloatLiteral(val) => LiteralValue::Float(val.clone()),
                    TokenVariant::StringLiteral(val) => LiteralValue::String(val.clone()),
                    TokenVariant::BoolLiteral(val) => LiteralValue::Bool(val.clone()),
                    _ => {
                        return Err(ParsingError::UnexpectedToken {
                            expected: "a literal".to_string(),
                            found: token.variant.clone(),
                            pos: token.pos,
                        });
                    }
                };

                Ok(Self { value, pos })
            }
            None => Err(ParsingError::UnexpectedEOF {
                expected: "a literal".to_string(),
            }),
        }
    }
}

impl Positioned for LiteralExpressionAtom {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl LiteralExpressionAtom {
    pub fn new_int(value: i64, line: usize, first_char: usize) -> Self {
        Self {
            value: LiteralValue::Int(value),
            pos: Position::new(line, first_char),
        }
    }

    pub fn new_float(value: f64, line: usize, first_char: usize) -> Self {
        Self {
            value: LiteralValue::Float(value),
            pos: Position::new(line, first_char),
        }
    }

    pub fn new_string(value: String, line: usize, first_char: usize) -> Self {
        Self {
            value: LiteralValue::String(value),
            pos: Position::new(line, first_char),
        }
    }

    pub fn new_bool(value: bool, line: usize, first_char: usize) -> Self {
        Self {
            value: LiteralValue::Bool(value),
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_parsing() {
        let mut ts = test_token_stream![
            TokenVariant::IntLiteral(42),
            TokenVariant::FloatLiteral(3.14),
            TokenVariant::StringLiteral("Hello".to_string()),
            TokenVariant::BoolLiteral(true)
        ];

        assert!(LiteralExpressionAtom::is_next(&ts));
        let int_literal = LiteralExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(int_literal, LiteralExpressionAtom::new_int(42, 0, 0));

        assert!(LiteralExpressionAtom::is_next(&ts));
        let float_literal = LiteralExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(float_literal, LiteralExpressionAtom::new_float(3.14, 1, 0));

        assert!(LiteralExpressionAtom::is_next(&ts));
        let string_literal = LiteralExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(
            string_literal,
            LiteralExpressionAtom::new_string("Hello".to_string(), 2, 0)
        );

        assert!(LiteralExpressionAtom::is_next(&ts));
        let bool_literal = LiteralExpressionAtom::parse(&mut ts).unwrap();
        assert_eq!(bool_literal, LiteralExpressionAtom::new_bool(true, 3, 0));
    }

    #[test]
    fn test_literal_parsing_errors() {
        let mut ts = test_token_stream![TokenVariant::Identifier("not_a_literal".to_string())];

        assert!(!LiteralExpressionAtom::is_next(&ts));
        let result = LiteralExpressionAtom::parse(&mut ts);
        assert!(result.is_err());
    }
}
