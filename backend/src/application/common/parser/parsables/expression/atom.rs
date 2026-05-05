use crate::{application::common::parser::{error::ParsingError, lexer::{token::{Token, TokenVariant}, token_stream::TokenStream}, macros::{get_pos, is_token}, parsable::Parsable}, domain::{common::position::{Position, Positioned}, game::{abstract_syntax_tree::expression::atom::ExpressionAtom, value_objects::data::Value}}};

impl Parsable for ExpressionAtom {
    fn is_next(ts: &TokenStream) -> bool {
        Value::is_next(ts)
            || is_token!(ts, TokenVariant::Identifier(_))
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        if Value::is_next(ts) {
            Ok(ExpressionAtom::Literal(Value::parse(ts, source_code)?, pos))
        } else if is_token!(ts, TokenVariant::Identifier(_)) {
            match ts.next() {
                Some(Token { variant: TokenVariant::Identifier(name), .. }) => Ok(ExpressionAtom::Variable(name.clone(), pos)),
                _ => unreachable!("Token wasn't a Identifier after checking it was one.")
            }
        } else {
            Err(ParsingError::SyntaxError {
                message: "Expected a literal or identifier as expression atom.".to_string(),
                pos,
            })
        }
    }
}

impl Positioned for ExpressionAtom {
    fn position(&self) -> Position {
        match self {
            ExpressionAtom::Literal(_, pos) => *pos,
            ExpressionAtom::Variable(_, pos) => *pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;

    use super::*;

    #[test]
    fn test_expression_atom_literal_int_parsing() {
        let (mut ts, source_code) = test_token_stream!("42");

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts, &source_code).unwrap();
        assert_eq!(atom, ExpressionAtom::Literal(Value::Int(42), Position::new(0, 0)));
    }

    #[test]
    fn test_expression_atom_literal_float_parsing() {
        let (mut ts, source_code) = test_token_stream!("3.14");

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts, &source_code).unwrap();
        assert_eq!(atom, ExpressionAtom::Literal(Value::Float(3.14), Position::new(0, 0)));
    }

    #[test]
    fn test_expression_atom_literal_string_parsing() {
        let (mut ts, source_code) = test_token_stream!("\"hello\"");

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts, &source_code).unwrap();
        assert_eq!(atom, ExpressionAtom::Literal(Value::String("hello".to_string()), Position::new(0, 0)));
    }

    #[test]
    fn test_expression_atom_literal_bool_parsing() {
        let (mut ts, source_code) = test_token_stream!("true");

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts, &source_code).unwrap();
        assert_eq!(atom, ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 0)));
    }

    #[test]
    fn test_expression_atom_variable_parsing() {
        let (mut ts, source_code) = test_token_stream!("myVariable");

        assert!(ExpressionAtom::is_next(&ts));
        let atom = ExpressionAtom::parse(&mut ts, &source_code).unwrap();
        assert_eq!(atom, ExpressionAtom::Variable("myVariable".to_string(), Position::new(0, 0)));
    }
}
