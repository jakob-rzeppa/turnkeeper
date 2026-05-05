use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token}, parsable::Parsable, parsables::expression::Expression}, domain::game::abstract_syntax_tree::expression::{atom::ExpressionAtom, unary::{UnaryExpression, UnaryOperator}}};


impl Parsable for UnaryExpression {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::Minus) || is_token!(ts, TokenVariant::Exclamation)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<UnaryExpression, ParsingError> {
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

            return Ok(UnaryExpression::new(operator, expr, pos));
        }

        // Otherwise, we can parse the next token as an expression atom and use that as the operand of the unary operator
        let operant = ExpressionAtom::parse(ts, source_code).map_err(|err| 
            change_err_msg!(err, "Expected expression atom or after '(' in unary operator")
        )?;

        Ok(UnaryExpression::new(operator, Expression::Atom(operant), pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::{application::common::parser::macros::test_token_stream, domain::common::position::Position};
    use super::*;

    #[test]
    fn test_logical_not_parsing() {
        let (mut ts, source_code) = test_token_stream!("!x");

        assert!(UnaryExpression::is_next(&ts));
        let unary_expr = UnaryExpression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            unary_expr,
            UnaryExpression::new(UnaryOperator::LogicalNot, Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 1))), Position::new(0, 0))
        );
    }

    #[test]
    fn test_negation_parsing() {
        let (mut ts, source_code) = test_token_stream!("-y");

        assert!(UnaryExpression::is_next(&ts));
        let unary_expr = UnaryExpression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            unary_expr,
            UnaryExpression::new(UnaryOperator::Negation, Expression::Atom(ExpressionAtom::Variable("y".to_string(), Position::new(0, 1))), Position::new(0, 0))
        );
    }

    #[test]
    fn test_parenthesized_unary_parsing() {
        let (mut ts, source_code) = test_token_stream!("!(z)");

        assert!(UnaryExpression::is_next(&ts));
        let unary_expr = UnaryExpression::parse(&mut ts, &source_code).unwrap();
        assert_eq!(
            unary_expr,
            UnaryExpression::new(UnaryOperator::LogicalNot, Expression::Atom(ExpressionAtom::Variable("z".to_string(), Position::new(0, 2))), Position::new(0, 0))
        );
    }
}
