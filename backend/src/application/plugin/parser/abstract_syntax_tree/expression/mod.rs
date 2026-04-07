use crate::application::plugin::{
    lexer::token::TokenVariant,
    parser::{
        abstract_syntax_tree::{
            Parsable, Positioned, TokenStream,
            expression::{
                atom::ExpressionAtom,
                binary::{BinaryExpression, BinaryOperator},
                unary::UnaryExpression,
            },
        },
        error::ParsingError,
    },
};

pub mod atom;
pub mod binary;
pub mod unary;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Atom(ExpressionAtom),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

impl Parsable for Expression {
    fn is_next(_ts: &TokenStream) -> bool {
        true
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        Self::pratt_parse(ts, 0)
    }
}

impl Expression {
    fn pratt_parse(ts: &mut TokenStream, min_bp: u8) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let mut left = if is_token!(ts, TokenVariant::LeftParen) {
            ts.next(); // consume '('

            let expr = Self::pratt_parse(ts, 0)?;

            expect_token!(
                ts,
                TokenVariant::RightParen,
                "')' after parenthesized expression"
            );

            expr
        } else if UnaryExpression::is_next(ts) {
            Expression::Unary(UnaryExpression::parse(ts)?)
        } else if ExpressionAtom::is_next(ts) {
            Expression::Atom(ExpressionAtom::parse(ts)?)
        } else {
            let next = ts.next();

            return match next {
                Some(t) => Err(ParsingError::UnexpectedToken {
                    expected: "identifier, literal, unary operator or '(' at the beginning of a expression".to_string(),
                    found: t.variant.clone(),
                    pos,
                }),
                None => Err(ParsingError::UnexpectedEOF {
                    expected: "identifier, literal, unary operator or '(' at the beginning of a expression".to_string(),
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

            let right = Self::pratt_parse(ts, r_bp)?;

            left = Expression::Binary(BinaryExpression::new(left, operator, right, pos));
        }

        Ok(left)
    }
}

impl Positioned for Expression {
    fn position(&self) -> crate::application::plugin::common::Position {
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
        use crate::application::plugin::common::Position;

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
    use super::*;

    // === Atom Tests ===

    #[test]
    fn test_literal_int() {
        let mut ts = test_token_stream!(TokenVariant::IntLiteral(42));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(parsed, Expression::new_atom_literal_int(42, 0, 0));
    }

    #[test]
    fn test_literal_float() {
        let mut ts = test_token_stream!(TokenVariant::FloatLiteral(3.14));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(parsed, Expression::new_atom_literal_float(3.14, 0, 0));
    }

    #[test]
    fn test_literal_string() {
        let mut ts = test_token_stream!(TokenVariant::StringLiteral("hello".to_string()));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_atom_literal_string("hello".to_string(), 0, 0)
        );
    }

    #[test]
    fn test_literal_bool() {
        let mut ts = test_token_stream!(TokenVariant::BoolLiteral(true));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(parsed, Expression::new_atom_literal_bool(true, 0, 0));
    }

    #[test]
    fn test_variable() {
        let mut ts = test_token_stream!(TokenVariant::Identifier("x".to_string()));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(parsed, Expression::new_atom_variable("x", 0, 0));
    }

    // === Unary Tests ===

    #[test]
    fn test_unary_negation() {
        let mut ts = test_token_stream!(TokenVariant::Minus, TokenVariant::IntLiteral(5));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_unary_negation(Expression::new_atom_literal_int(5, 1, 0), 0, 0)
        );
    }

    #[test]
    fn test_unary_not() {
        let mut ts = test_token_stream!(TokenVariant::Not, TokenVariant::BoolLiteral(true));

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_unary_logical_not(Expression::new_atom_literal_bool(true, 1, 0), 0, 0)
        );
    }

    // === Binary Tests ===

    #[test]
    fn test_binary_addition() {
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(1),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(2)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(1, 0, 0),
                BinaryOperator::Addition,
                Expression::new_atom_literal_int(2, 2, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_binary_multiplication() {
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(3),
            TokenVariant::Star,
            TokenVariant::IntLiteral(4)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(3, 0, 0),
                BinaryOperator::Multiplication,
                Expression::new_atom_literal_int(4, 2, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_binary_comparison() {
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(5),
            TokenVariant::LessEqual,
            TokenVariant::IntLiteral(10)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(5, 0, 0),
                BinaryOperator::LessThanOrEqual,
                Expression::new_atom_literal_int(10, 2, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_binary_logical_and() {
        let mut ts = test_token_stream!(
            TokenVariant::BoolLiteral(true),
            TokenVariant::And,
            TokenVariant::BoolLiteral(false)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_bool(true, 0, 0),
                BinaryOperator::LogicalAnd,
                Expression::new_atom_literal_bool(false, 2, 0),
                0,
                0
            )
        );
    }

    // === Precedence Tests ===

    #[test]
    fn test_precedence_mul_over_add() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(1),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(2),
            TokenVariant::Star,
            TokenVariant::IntLiteral(3)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(1, 0, 0),
                BinaryOperator::Addition,
                Expression::new_binary(
                    Expression::new_atom_literal_int(2, 2, 0),
                    BinaryOperator::Multiplication,
                    Expression::new_atom_literal_int(3, 4, 0),
                    2,
                    0
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_precedence_mul_before_add() {
        // 2 * 3 + 1 should parse as (2 * 3) + 1
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(2),
            TokenVariant::Star,
            TokenVariant::IntLiteral(3),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(1)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(2, 0, 0),
                    BinaryOperator::Multiplication,
                    Expression::new_atom_literal_int(3, 2, 0),
                    0,
                    0
                ),
                BinaryOperator::Addition,
                Expression::new_atom_literal_int(1, 4, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_left_associativity() {
        // 1 - 2 - 3 should parse as (1 - 2) - 3
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(1),
            TokenVariant::Minus,
            TokenVariant::IntLiteral(2),
            TokenVariant::Minus,
            TokenVariant::IntLiteral(3)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 0, 0),
                    BinaryOperator::Subtraction,
                    Expression::new_atom_literal_int(2, 2, 0),
                    0,
                    0
                ),
                BinaryOperator::Subtraction,
                Expression::new_atom_literal_int(3, 4, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_right_associativity_power() {
        // 2 ^ 3 ^ 4 should parse as 2 ^ (3 ^ 4)
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(2),
            TokenVariant::Caret,
            TokenVariant::IntLiteral(3),
            TokenVariant::Caret,
            TokenVariant::IntLiteral(4)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(2, 0, 0),
                BinaryOperator::Power,
                Expression::new_binary(
                    Expression::new_atom_literal_int(3, 2, 0),
                    BinaryOperator::Power,
                    Expression::new_atom_literal_int(4, 4, 0),
                    2,
                    0
                ),
                0,
                0
            )
        );
    }

    // === Parentheses Tests ===

    #[test]
    fn test_parentheses_override_precedence() {
        // (1 + 2) * 3 should parse as (1 + 2) * 3
        let mut ts = test_token_stream!(
            TokenVariant::LeftParen,
            TokenVariant::IntLiteral(1),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(2),
            TokenVariant::RightParen,
            TokenVariant::Star,
            TokenVariant::IntLiteral(3)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 1, 0),
                    BinaryOperator::Addition,
                    Expression::new_atom_literal_int(2, 3, 0),
                    1,
                    0
                ),
                BinaryOperator::Multiplication,
                Expression::new_atom_literal_int(3, 6, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_nested_parentheses() {
        // ((1 + 2))
        let mut ts = test_token_stream!(
            TokenVariant::LeftParen,
            TokenVariant::LeftParen,
            TokenVariant::IntLiteral(1),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(2),
            TokenVariant::RightParen,
            TokenVariant::RightParen
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_atom_literal_int(1, 2, 0),
                BinaryOperator::Addition,
                Expression::new_atom_literal_int(2, 4, 0),
                2,
                0
            )
        );
    }

    // === Complex Expression Tests ===

    #[test]
    fn test_complex_arithmetic() {
        // 1 + 2 * 3 - 4 / 2 should parse as (1 + (2 * 3)) - (4 / 2)
        let mut ts = test_token_stream!(
            TokenVariant::IntLiteral(1),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(2),
            TokenVariant::Star,
            TokenVariant::IntLiteral(3),
            TokenVariant::Minus,
            TokenVariant::IntLiteral(4),
            TokenVariant::Slash,
            TokenVariant::IntLiteral(2)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_literal_int(1, 0, 0),
                    BinaryOperator::Addition,
                    Expression::new_binary(
                        Expression::new_atom_literal_int(2, 2, 0),
                        BinaryOperator::Multiplication,
                        Expression::new_atom_literal_int(3, 4, 0),
                        2,
                        0
                    ),
                    0,
                    0
                ),
                BinaryOperator::Subtraction,
                Expression::new_binary(
                    Expression::new_atom_literal_int(4, 6, 0),
                    BinaryOperator::Division,
                    Expression::new_atom_literal_int(2, 8, 0),
                    6,
                    0
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_complex_logical() {
        // a && b || c should parse as (a && b) || c
        let mut ts = test_token_stream!(
            TokenVariant::Identifier("a".to_string()),
            TokenVariant::And,
            TokenVariant::Identifier("b".to_string()),
            TokenVariant::Or,
            TokenVariant::Identifier("c".to_string())
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_variable("a", 0, 0),
                    BinaryOperator::LogicalAnd,
                    Expression::new_atom_variable("b", 2, 0),
                    0,
                    0
                ),
                BinaryOperator::LogicalOr,
                Expression::new_atom_variable("c", 4, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_comparison_with_arithmetic() {
        // x + 1 < y * 2 should parse as (x + 1) < (y * 2)
        let mut ts = test_token_stream!(
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Plus,
            TokenVariant::IntLiteral(1),
            TokenVariant::Less,
            TokenVariant::Identifier("y".to_string()),
            TokenVariant::Star,
            TokenVariant::IntLiteral(2)
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 0, 0),
                    BinaryOperator::Addition,
                    Expression::new_atom_literal_int(1, 2, 0),
                    0,
                    0
                ),
                BinaryOperator::LessThan,
                Expression::new_binary(
                    Expression::new_atom_variable("y", 4, 0),
                    BinaryOperator::Multiplication,
                    Expression::new_atom_literal_int(2, 6, 0),
                    4,
                    0
                ),
                0,
                0
            )
        );
    }

    #[test]
    fn test_unary_in_binary() {
        // -a + b
        let mut ts = test_token_stream!(
            TokenVariant::Minus,
            TokenVariant::Identifier("a".to_string()),
            TokenVariant::Plus,
            TokenVariant::Identifier("b".to_string())
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_unary_negation(Expression::new_atom_variable("a", 1, 0), 0, 0),
                BinaryOperator::Addition,
                Expression::new_atom_variable("b", 3, 0),
                0,
                0
            )
        );
    }

    #[test]
    fn test_full_expression() {
        // (a + b) * c == d && !e
        let mut ts = test_token_stream!(
            TokenVariant::LeftParen,
            TokenVariant::Identifier("a".to_string()),
            TokenVariant::Plus,
            TokenVariant::Identifier("b".to_string()),
            TokenVariant::RightParen,
            TokenVariant::Star,
            TokenVariant::Identifier("c".to_string()),
            TokenVariant::EqualEqual,
            TokenVariant::Identifier("d".to_string()),
            TokenVariant::And,
            TokenVariant::Not,
            TokenVariant::Identifier("e".to_string())
        );

        assert!(Expression::is_next(&ts));
        let parsed = Expression::parse(&mut ts).unwrap();
        assert_eq!(
            parsed,
            Expression::new_binary(
                Expression::new_binary(
                    Expression::new_binary(
                        Expression::new_binary(
                            Expression::new_atom_variable("a", 1, 0),
                            BinaryOperator::Addition,
                            Expression::new_atom_variable("b", 3, 0),
                            1,
                            0
                        ),
                        BinaryOperator::Multiplication,
                        Expression::new_atom_variable("c", 6, 0),
                        0,
                        0
                    ),
                    BinaryOperator::Equal,
                    Expression::new_atom_variable("d", 8, 0),
                    0,
                    0
                ),
                BinaryOperator::LogicalAnd,
                Expression::new_unary_logical_not(Expression::new_atom_variable("e", 11, 0), 10, 0),
                0,
                0
            )
        );
    }
}
