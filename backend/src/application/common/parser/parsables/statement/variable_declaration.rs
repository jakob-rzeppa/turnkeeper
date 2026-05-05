use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token}, parsable::Parsable}, domain::game::{abstract_syntax_tree::{expression::Expression, statement::VariableDeclarationStatement}, value_objects::data::Datatype}};



impl Parsable for VariableDeclarationStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Let)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(
            ts,
            TokenVariant::Let,
            "Expected 'let' keyword at the beginning of variable declaration"
        );

        let name = match ts.next() {
            Some(t) if matches!(t.variant, TokenVariant::Identifier(_)) => {
                if let TokenVariant::Identifier(name) = &t.variant {
                    name.clone()
                } else {
                    unreachable!("Token wasn't an Identifier after checking it was one.")
                }
            }
            Some(t) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected identifier after 'let' in variable declaration".to_string(),
                    found: t.variant.clone(),
                    pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "Expected identifier after 'let' in variable declaration".to_string(),
                });
            }
        };

        expect_token!(
            ts,
            TokenVariant::Colon,
            "':' after identifier in variable declaration"
        );

        let var_type = Datatype::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected variable type (int, float, string, bool) in variable declaration")
        )?;

        expect_token!(
            ts,
            TokenVariant::Equal,
            "Expected '=' after datatype in variable declaration"
        );

        let value = Expression::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected expression after '=' in variable declaration")
        )?;

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "Expected ';' at the end of variable declaration"
        );

        Ok(VariableDeclarationStatement::new(name, var_type, value, pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::{application::common::parser::macros::test_token_stream, domain::{common::position::Position, game::{abstract_syntax_tree::expression::{atom::ExpressionAtom, binary::{BinaryExpression, BinaryOperator}}, value_objects::data::Value}}};

use super::*;

    #[test]
    fn test_variable_declaration_int_parsing() {
        let (mut ts, source_code) = test_token_stream!("let x: int = 42;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "x".to_string(),
                Datatype::Int,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(42), Position::new(0, 13))),
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_variable_declaration_float_parsing() {
        let (mut ts, source_code) = test_token_stream!("let pi: float = 3.14;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "pi".to_string(),
                Datatype::Float,
                Expression::Atom(ExpressionAtom::Literal(Value::Float(3.14), Position::new(0, 16))),
                Position::new(0, 0) 
            )
        );
    }

    #[test]
    fn test_variable_declaration_string_parsing() {
        let (mut ts, source_code) = test_token_stream!("let name: string = \"Hello\";");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "name".to_string(),
                Datatype::String,
                Expression::Atom(ExpressionAtom::Literal(Value::String("Hello".to_string()), Position::new(0, 19))),
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_variable_declaration_bool_parsing() {
        let (mut ts, source_code) = test_token_stream!("let flag: bool = true;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "flag".to_string(),
                Datatype::Bool,
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 17))),
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_variable_declaration_with_expression_parsing() {
        let (mut ts, source_code) = test_token_stream!("let sum: int = 1 + 2;");

        assert!(VariableDeclarationStatement::is_next(&ts));
        let stmt = VariableDeclarationStatement::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            stmt,
            VariableDeclarationStatement::new(
                "sum".to_string(),
                Datatype::Int,
                Expression::Binary(
                    BinaryExpression::new(
                        Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 15))),
                        BinaryOperator::Addition,
                        Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 19))),
                        Position::new(0, 15)
                    )
                ),
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_variable_declaration_not_next_on_non_let() {
        let (ts, _source_code) = test_token_stream!("x");

        assert!(!VariableDeclarationStatement::is_next(&ts));
    }
}
