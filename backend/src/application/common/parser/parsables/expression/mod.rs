use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{expect_token, get_pos, is_token}, parsable::Parsable}, domain::{common::position::{Position, Positioned}, game::{abstract_syntax_tree::expression::{Expression, atom::ExpressionAtom, binary::{BinaryExpression, BinaryOperator}, unary::UnaryExpression}, value_objects::data::Value}}};

pub mod atom;
pub mod binary;
pub mod unary;

impl Parsable for Expression {
    fn is_next(_ts: &TokenStream) -> bool {
        true
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        Self::pratt_parse(ts, source_code, 0)
    }
}

impl Expression {
    fn pratt_parse(ts: &mut TokenStream, source_code: &str, min_bp: u8) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let mut left = if is_token!(ts, TokenVariant::OpenParen) {
            ts.next(); // consume '('

            let expr = Self::pratt_parse(ts, source_code, 0)?;

            expect_token!(
                ts,
                TokenVariant::CloseParen,
                "Expected ')' after parenthesized expression"
            );

            expr
        } else if UnaryExpression::is_next(ts) {
            Expression::Unary(UnaryExpression::parse(ts, source_code)?)
        } else if ExpressionAtom::is_next(ts) {
            Expression::Atom(ExpressionAtom::parse(ts, source_code)?)
        } else {
            let next = ts.next();

            return match next {
                Some(t) => Err(ParsingError::UnexpectedToken {
                    expected: "Expected identifier, literal, unary operator or '(' at the beginning of a expression".to_string(),
                    found: t.variant.clone(),
                    pos,
                }),
                None => Err(ParsingError::UnexpectedEOF {
                    expected: "Expected identifier, literal, unary operator or '(' at the beginning of a expression".to_string(),
                }),
            };
        };

        loop {
            // Do not consume the operator yet, we need to check its binding power first
            let operator = match ts.peek() {
                Some(t) => match BinaryOperator::from_token(t) {
                    Some(op) => op,
                    None => break,
                },
                _ => break,
            };

            let (l_bp, r_bp) = operator.binding_power();
            if l_bp < min_bp {
                break;
            }
            ts.next(); // Now we can consume the operator

            let right = Self::pratt_parse(ts, source_code, r_bp)?;

            left = Expression::Binary(BinaryExpression::new(left, operator, right, pos));
        }

        Ok(left)
    }
}

impl Positioned for Expression {
    fn position(&self) -> Position {
        match self {
            Expression::Atom(atom) => atom.position(),
            Expression::Unary(unary) => unary.position(),
            Expression::Binary(binary) => binary.position(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{application::common::parser::macros::test_token_stream, domain::game::abstract_syntax_tree::expression::unary::UnaryOperator};
    use super::*;

    // === Atom Tests ===

    #[test]
    fn test_literal_int() {
        let (mut ts, source_code) = test_token_stream!("42");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::Atom(ExpressionAtom::Literal(Value::Int(42), Position::new(0, 0))));
    }

    #[test]
    fn test_literal_float() {
        let (mut ts, source_code) = test_token_stream!("3.14");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::Atom(ExpressionAtom::Literal(Value::Float(3.14), Position::new(0, 0))));
    }

    #[test]
    fn test_literal_string() {
        let (mut ts, source_code) = test_token_stream!("\"hello\"");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Atom(ExpressionAtom::Literal(Value::String("hello".to_string()), Position::new(0, 0)))
        );
    }

    #[test]
    fn test_literal_bool() {
        let (mut ts, source_code) = test_token_stream!("true");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::Atom(ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 0))));
    }

    #[test]
    fn test_variable() {
        let (mut ts, source_code) = test_token_stream!("x");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 0))));
    }

    // === Unary Tests ===

    #[test]
    fn test_unary_negation() {
        let (mut ts, source_code) = test_token_stream!("-5");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Unary(UnaryExpression::new(UnaryOperator::Negation, Expression::Atom(ExpressionAtom::Literal(Value::Int(5), Position::new(0, 1))), Position::new(0, 0)))
        );
    }

    #[test]
    fn test_unary_not() {
        let (mut ts, source_code) = test_token_stream!("!true");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Unary(UnaryExpression::new(UnaryOperator::LogicalNot, Expression::Atom(ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 1))), Position::new(0, 0)))
        );
    }

    // === Binary Tests ===

    #[test]
    fn test_binary_addition() {
        let (mut ts, source_code) = test_token_stream!("1 + 2");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 0))),
                BinaryOperator::Addition,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 4))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_binary_multiplication() {
        let (mut ts, source_code) = test_token_stream!("3 * 4");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 0))),
                BinaryOperator::Multiplication,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(4), Position::new(0, 4))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_binary_comparison() {
        let (mut ts, source_code) = test_token_stream!("5 <= 10");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Int(5), Position::new(0, 0))),
                BinaryOperator::LessThanOrEqual,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(10), Position::new(0, 5))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_binary_logical_and() {
        let (mut ts, source_code) = test_token_stream!("true && false");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 0))),
                BinaryOperator::LogicalAnd,
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(false), Position::new(0, 8))),
                Position::new(0, 0)
            ))
        );
    }

    // === Precedence Tests ===

    #[test]
    fn test_precedence_mul_over_add() {
        let (mut ts, source_code) = test_token_stream!("1 + 2 * 3");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 0))),
                BinaryOperator::Addition,
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 4))),
                    BinaryOperator::Multiplication,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 8))),
                    Position::new(0, 4)
                )),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_precedence_mul_before_add() {
        let (mut ts, source_code) = test_token_stream!("2 * 3 + 1");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 0))),
                    BinaryOperator::Multiplication,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 4))),
                    Position::new(0, 0)
                )),
                BinaryOperator::Addition,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 8))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_left_associativity() {
        let (mut ts, source_code) = test_token_stream!("1 - 2 - 3");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 0))),
                    BinaryOperator::Subtraction,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 4))),
                    Position::new(0, 0)
                )),
                BinaryOperator::Subtraction,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 8))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_right_associativity_power() {
        let (mut ts, source_code) = test_token_stream!("2 ^ 3 ^ 4");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 0))),
                BinaryOperator::Power,
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 4))),
                    BinaryOperator::Power,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(4), Position::new(0, 8))),
                    Position::new(0, 4)
                )),
                Position::new(0, 0)
            ))
        );
    }

    // === Parentheses Tests ===

    #[test]
    fn test_parentheses_override_precedence() {
        let (mut ts, source_code) = test_token_stream!("(1 + 2) * 3");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 1))),
                    BinaryOperator::Addition,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 5))),
                    Position::new(0, 1)
                )),
                BinaryOperator::Multiplication,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 10))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_nested_parentheses() {
        let (mut ts, source_code) = test_token_stream!("((1 + 2))");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 2))),
                BinaryOperator::Addition,
                Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 6))),
                Position::new(0, 2)
            ))
        );
    }

    // === Complex Expression Tests ===

    #[test]
    fn test_complex_arithmetic() {
        let (mut ts, source_code) = test_token_stream!("1 + 2 * 3 - 4 / 2");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 0))),
                    BinaryOperator::Addition,
                    Expression::Binary(BinaryExpression::new(
                        Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 4))),
                        BinaryOperator::Multiplication,
                        Expression::Atom(ExpressionAtom::Literal(Value::Int(3), Position::new(0, 8))),
                        Position::new(0, 4)
                    )),
                    Position::new(0, 0)
                )),
                BinaryOperator::Subtraction,
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(4), Position::new(0, 12))),
                    BinaryOperator::Division,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 16))),
                    Position::new(0, 12)
                )),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_complex_logical() {
        let (mut ts, source_code) = test_token_stream!("a && b || c");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("a".to_string(), Position::new(0, 0))),
                    BinaryOperator::LogicalAnd,
                    Expression::Atom(ExpressionAtom::Variable("b".to_string(), Position::new(0, 5))),
                    Position::new(0, 0)
                )),
                BinaryOperator::LogicalOr,
                Expression::Atom(ExpressionAtom::Variable("c".to_string(), Position::new(0, 10))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_comparison_with_arithmetic() {
        let (mut ts, source_code) = test_token_stream!("x + 1 < y * 2");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 0))),
                    BinaryOperator::Addition,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 4))),
                    Position::new(0, 0)
                )),
                BinaryOperator::LessThan,
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("y".to_string(), Position::new(0, 8))),
                    BinaryOperator::Multiplication,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(2), Position::new(0, 12))),
                    Position::new(0, 8)
                )),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_unary_in_binary() {
        let (mut ts, source_code) = test_token_stream!("-a + b");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Unary(UnaryExpression::new(UnaryOperator::Negation, Expression::Atom(ExpressionAtom::Variable("a".to_string(), Position::new(0, 1))), Position::new(0, 0))),
                BinaryOperator::Addition,
                Expression::Atom(ExpressionAtom::Variable("b".to_string(), Position::new(0, 5))),
                Position::new(0, 0)
            ))
        );
    }

    #[test]
    fn test_full_expression() {
        let (mut ts, source_code) = test_token_stream!("(a + b) * c == d && !e");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::Binary(BinaryExpression::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Binary(BinaryExpression::new(
                        Expression::Binary(BinaryExpression::new(
                            Expression::Atom(ExpressionAtom::Variable("a".to_string(), Position::new(0, 1))),
                            BinaryOperator::Addition,
                            Expression::Atom(ExpressionAtom::Variable("b".to_string(), Position::new(0, 5))),
                            Position::new(0, 1)
                        )),
                        BinaryOperator::Multiplication,
                        Expression::Atom(ExpressionAtom::Variable("c".to_string(), Position::new(0, 10))),
                        Position::new(0, 0)
                    )),
                    BinaryOperator::Equal,
                    Expression::Atom(ExpressionAtom::Variable("d".to_string(), Position::new(0, 15))),
                    Position::new(0, 0)
                )),
                BinaryOperator::LogicalAnd,
                Expression::Unary(UnaryExpression::new(UnaryOperator::LogicalNot, Expression::Atom(ExpressionAtom::Variable("e".to_string(), Position::new(0, 21))), Position::new(0, 20))),
                Position::new(0, 0)
            ))
        );
    }
}
