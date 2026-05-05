use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{expect_token, get_pos, is_token}, parsable::Parsable, parsables::expression::{atom::ExpressionAtom, binary::{BinaryExpression, BinaryOperator}, unary::UnaryExpression}}, domain::common::position::{Position, Positioned}};

pub mod atom;
pub mod binary;
pub mod unary;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expression {
    Atom(ExpressionAtom),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

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
impl Expression {
    pub fn new_atom_literal_int(value: i64, line: usize, first_char: usize) -> Self {
        Expression::Atom(ExpressionAtom::new_literal_int(value, line, first_char))
    }

    pub fn new_atom_literal_float(value: f64, line: usize, first_char: usize) -> Self {
        Expression::Atom(ExpressionAtom::new_literal_float(value, line, first_char))
    }

    pub fn new_atom_literal_string(value: String, line: usize, first_char: usize) -> Self {
        Expression::Atom(ExpressionAtom::new_literal_string(value, line, first_char))
    }

    pub fn new_atom_literal_bool(value: bool, line: usize, first_char: usize) -> Self {
        Expression::Atom(ExpressionAtom::new_literal_bool(value, line, first_char))
    }

    pub fn new_atom_variable(name: &str, line: usize, first_char: usize) -> Self {
        Expression::Atom(ExpressionAtom::new_variable(name, line, first_char))
    }

    pub fn new_unary_negation(operand: Expression, line: usize, first_char: usize) -> Self {
        Expression::Unary(UnaryExpression::new_negation(operand, line, first_char))
    }

    pub fn new_unary_logical_not(operand: Expression, line: usize, first_char: usize) -> Self {
        Expression::Unary(UnaryExpression::new_logical_not(operand, line, first_char))
    }

    pub fn new_binary(
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
        line: usize,
        first_char: usize,
    ) -> Self {
        use Position;

        Expression::Binary(BinaryExpression::new(
            left,
            operator,
            right,
            Position::new(line, first_char),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;
    use super::*;

    // === Atom Tests ===

    #[test]
    fn test_literal_int() {
        let (mut ts, source_code) = test_token_stream!("42");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::new_atom_literal_int(42, 0, 0));
    }

    #[test]
    fn test_literal_float() {
        let (mut ts, source_code) = test_token_stream!("3.14");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::new_atom_literal_float(3.14, 0, 0));
    }

    #[test]
    fn test_literal_string() {
        let (mut ts, source_code) = test_token_stream!("\"hello\"");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_atom_literal_string("hello".to_string(), 0, 0)
        );
    }

    #[test]
    fn test_literal_bool() {
        let (mut ts, source_code) = test_token_stream!("true");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::new_atom_literal_bool(true, 0, 0));
    }

    #[test]
    fn test_variable() {
        let (mut ts, source_code) = test_token_stream!("x");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(parsed, Expression::new_atom_variable("x", 0, 0));
    }

    // === Unary Tests ===

    #[test]
    fn test_unary_negation() {
        let (mut ts, source_code) = test_token_stream!("-5");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_unary_negation(Expression::new_atom_literal_int(5, 0, 1), 0, 0)
        );
    }

    #[test]
    fn test_unary_not() {
        let (mut ts, source_code) = test_token_stream!("!true");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_unary_logical_not(Expression::new_atom_literal_bool(true, 0, 1), 0, 0)
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
            Expression::new_binary(
                Expression::new_atom_literal_int(1, 0, 0),
                BinaryOperator::Addition,
                Expression::new_atom_literal_int(2, 0, 4),
                0,
                0
            )
        );
    }

    #[test]
    fn test_binary_multiplication() {
        let (mut ts, source_code) = test_token_stream!("3 * 4");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(3, 0, 0),
                BinaryOperator::Multiplication,
                Expression::new_atom_literal_int(4, 0, 4),
                0,
                0
            )
        );
    }

    #[test]
    fn test_binary_comparison() {
        let (mut ts, source_code) = test_token_stream!("5 <= 10");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(5, 0, 0),
                BinaryOperator::LessThanOrEqual,
                Expression::new_atom_literal_int(10, 0, 5),
                0,
                0
            )
        );
    }

    #[test]
    fn test_binary_logical_and() {
        let (mut ts, source_code) = test_token_stream!("true && false");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_bool(true, 0, 0),
                BinaryOperator::LogicalAnd,
                Expression::new_atom_literal_bool(false, 0, 8),
                0,
                0
            )
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
            Expression::new_binary(
                Expression::new_atom_literal_int(1, 0, 0),
                BinaryOperator::Addition,
                Expression::new_binary(
                    Expression::new_atom_literal_int(2, 0, 4),
                    BinaryOperator::Multiplication,
                    Expression::new_atom_literal_int(3, 0, 8),
                    0,
                    4
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_precedence_mul_before_add() {
        let (mut ts, source_code) = test_token_stream!("2 * 3 + 1");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(2, 0, 0),
                    BinaryOperator::Multiplication,
                    Expression::new_atom_literal_int(3, 0, 4),
                    0,
                    0
                ),
                BinaryOperator::Addition,
                Expression::new_atom_literal_int(1, 0, 8),
                0,
                0
            )
        );
    }

    #[test]
    fn test_left_associativity() {
        let (mut ts, source_code) = test_token_stream!("1 - 2 - 3");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 0, 0),
                    BinaryOperator::Subtraction,
                    Expression::new_atom_literal_int(2, 0, 4),
                    0,
                    0
                ),
                BinaryOperator::Subtraction,
                Expression::new_atom_literal_int(3, 0, 8),
                0,
                0
            )
        );
    }

    #[test]
    fn test_right_associativity_power() {
        let (mut ts, source_code) = test_token_stream!("2 ^ 3 ^ 4");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(2, 0, 0),
                BinaryOperator::Power,
                Expression::new_binary(
                    Expression::new_atom_literal_int(3, 0, 4),
                    BinaryOperator::Power,
                    Expression::new_atom_literal_int(4, 0, 8),
                    0,
                    4
                ),
                0,
                0
            )
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
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 0, 1),
                    BinaryOperator::Addition,
                    Expression::new_atom_literal_int(2, 0, 5),
                    0,
                    1
                ),
                BinaryOperator::Multiplication,
                Expression::new_atom_literal_int(3, 0, 10),
                0,
                0
            )
        );
    }

    #[test]
    fn test_nested_parentheses() {
        let (mut ts, source_code) = test_token_stream!("((1 + 2))");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(1, 0, 2),
                BinaryOperator::Addition,
                Expression::new_atom_literal_int(2, 0, 6),
                0,
                2
            )
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
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 0, 0),
                    BinaryOperator::Addition,
                    Expression::new_binary(
                        Expression::new_atom_literal_int(2, 0, 4),
                        BinaryOperator::Multiplication,
                        Expression::new_atom_literal_int(3, 0, 8),
                        0,
                        4
                    ),
                    0,
                    0
                ),
                BinaryOperator::Subtraction,
                Expression::new_binary(
                    Expression::new_atom_literal_int(4, 0, 12),
                    BinaryOperator::Division,
                    Expression::new_atom_literal_int(2, 0, 16),
                    0,
                    12
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_complex_logical() {
        let (mut ts, source_code) = test_token_stream!("a && b || c");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_variable("a", 0, 0),
                    BinaryOperator::LogicalAnd,
                    Expression::new_atom_variable("b", 0, 5),
                    0,
                    0
                ),
                BinaryOperator::LogicalOr,
                Expression::new_atom_variable("c", 0, 10),
                0,
                0
            )
        );
    }

    #[test]
    fn test_comparison_with_arithmetic() {
        let (mut ts, source_code) = test_token_stream!("x + 1 < y * 2");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 0, 0),
                    BinaryOperator::Addition,
                    Expression::new_atom_literal_int(1, 0, 4),
                    0,
                    0
                ),
                BinaryOperator::LessThan,
                Expression::new_binary(
                    Expression::new_atom_variable("y", 0, 8),
                    BinaryOperator::Multiplication,
                    Expression::new_atom_literal_int(2, 0, 12),
                    0,
                    8
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_unary_in_binary() {
        let (mut ts, source_code) = test_token_stream!("-a + b");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_unary_negation(Expression::new_atom_variable("a", 0, 1), 0, 0),
                BinaryOperator::Addition,
                Expression::new_atom_variable("b", 0, 5),
                0,
                0
            )
        );
    }

    #[test]
    fn test_full_expression() {
        let (mut ts, source_code) = test_token_stream!("(a + b) * c == d && !e");

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_binary(
                        Expression::new_binary(
                            Expression::new_atom_variable("a", 0, 1),
                            BinaryOperator::Addition,
                            Expression::new_atom_variable("b", 0, 5),
                            0,
                            1
                        ),
                        BinaryOperator::Multiplication,
                        Expression::new_atom_variable("c", 0, 10),
                        0,
                        0
                    ),
                    BinaryOperator::Equal,
                    Expression::new_atom_variable("d", 0, 15),
                    0,
                    0
                ),
                BinaryOperator::LogicalAnd,
                Expression::new_unary_logical_not(Expression::new_atom_variable("e", 0, 21), 0, 20),
                0,
                0
            )
        );
    }
}
