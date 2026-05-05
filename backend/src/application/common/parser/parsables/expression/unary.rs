use std::fmt::Display;

use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token}, parsable::Parsable, parsables::expression::{Expression, atom::ExpressionAtom}}, domain::common::position::{Position, Positioned}};


#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    operator: UnaryOperator,
    operand: Box<Expression>,
    pos: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negation,
    LogicalNot,
}

impl UnaryExpression {
    pub fn operator(&self) -> &UnaryOperator {
        &self.operator
    }

    pub fn operand(&self) -> &Expression {
        &self.operand
    }
}

impl Parsable for UnaryExpression {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Minus) || is_token!(ts, TokenVariant::Exclamation)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let operator = match ts.next() {
            Some(t) if t.variant == TokenVariant::Minus => UnaryOperator::Negation,
            Some(t) if t.variant == TokenVariant::Exclamation => UnaryOperator::LogicalNot,
            Some(t) => {
                return Err(ParsingError::UnexpectedToken {
                    expected: "Expected unary operator".to_string(),
                    found: t.variant.clone(),
                    pos,
                });
            }
            None => {
                return Err(ParsingError::UnexpectedEOF {
                    expected: "unary operator".to_string(),
                });
            }
        };

        // If the next token is a left parenthesis, we need to parse the entire parenthesized expression as the operand of the unary operator
        if is_token!(ts, TokenVariant::OpenParen) {
            ts.next(); // consume '('

            let expr = Expression::parse(ts, source_code).map_err(|err| 
                change_err_msg!(err, "Expected expression after '(' in unary operator")
            )?;

            expect_token!(
                ts,
                TokenVariant::CloseParen,
                "Expected ')' after parenthesized expression"
            );

            return Ok(UnaryExpression {
                operator,
                operand: Box::new(expr),
                pos,
            });
        }

        // Otherwise, we can parse the next token as an expression atom and use that as the operand of the unary operator
        let operant = ExpressionAtom::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected expression atom or after '(' in unary operator")
        )?;

        Ok(UnaryExpression {
            operator,
            operand: Box::new(Expression::Atom(operant)),
            pos,
        })
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Negation => write!(f, "-"),
            UnaryOperator::LogicalNot => write!(f, "!"),
        }
    }
}

impl Positioned for UnaryExpression {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl UnaryExpression {
    pub fn new_logical_not(operand: Expression, line: usize, first_char: usize) -> Self {
        UnaryExpression {
            operator: UnaryOperator::LogicalNot,
            operand: Box::new(operand),
            pos: Position::new(line, first_char),
        }
    }

    pub fn new_negation(operand: Expression, line: usize, first_char: usize) -> Self {
        UnaryExpression {
            operator: UnaryOperator::Negation,
            operand: Box::new(operand),
            pos: Position::new(line, first_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::common::parser::macros::test_token_stream;
    use super::*;

    #[test]
    fn test_logical_not_parsing() {
        let (mut ts, source_code) = test_token_stream!("!x");

        assert!(UnaryExpression::is_next(&ts));
        let unary_expr = UnaryExpression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            unary_expr,
            UnaryExpression::new_logical_not(Expression::new_atom_variable("x", 0, 1), 0, 0)
        );
    }

    #[test]
    fn test_negation_parsing() {
        let (mut ts, source_code) = test_token_stream!("-y");

        assert!(UnaryExpression::is_next(&ts));
        let unary_expr = UnaryExpression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            unary_expr,
            UnaryExpression::new_negation(Expression::new_atom_variable("y", 0, 1), 0, 0)
        );
    }

    #[test]
    fn test_parenthesized_unary_parsing() {
        let (mut ts, source_code) = test_token_stream!("!(z)");

        assert!(UnaryExpression::is_next(&ts));
        let unary_expr = UnaryExpression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            unary_expr,
            UnaryExpression::new_logical_not(Expression::new_atom_variable("z", 0, 2), 0, 0)
        );
    }
}
