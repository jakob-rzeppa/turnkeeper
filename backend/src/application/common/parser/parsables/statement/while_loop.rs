use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{expect_token, get_pos, is_token}, parsable::Parsable, parsables::statement::Statement}, domain::{game::abstract_syntax_tree::{statement::WhileLoopStatement, expression::Expression}}};



impl Parsable for WhileLoopStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::While)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(ts, TokenVariant::While, "Expected 'while' keyword");

        let condition = Expression::parse(ts, source_code)?;

        expect_token!(ts, TokenVariant::OpenBrace, "Expected '{' after while condition");

        let mut body = Vec::new();
        while !is_token!(ts, TokenVariant::CloseBrace) {
            body.push(Statement::parse(ts, source_code)?);
        }

        expect_token!(ts, TokenVariant::CloseBrace, "Expected '}' to close while loop body");

        Ok(WhileLoopStatement::new(condition, body, pos))
    }
}

#[cfg(test)]
mod tests {
    use crate::{application::common::parser::macros::test_token_stream, domain::{common::position::Position, game::{abstract_syntax_tree::{expression::{atom::ExpressionAtom, binary::{BinaryExpression, BinaryOperator}}, statement::AssignmentStatement}, value_objects::data::Value}}};
    use super::*;

    #[test]
    fn test_parse_while_loop() {
        let (mut ts, source_code) = test_token_stream!("while x < 10 { x = 1; }");

        let while_loop = WhileLoopStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            while_loop,
            WhileLoopStatement::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 6))),
                    BinaryOperator::LessThan,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(10), Position::new(0, 10))),
                    Position::new(0, 6)
                )),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "x".to_string(),
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(1), Position::new(0, 19))),
                    Position::new(0, 15)
                ))],
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_parse_while_loop_empty_body() {
        let (mut ts, source_code) = test_token_stream!("while x > 0 { }");

        let while_loop = WhileLoopStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            while_loop,
            WhileLoopStatement::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 6))),
                    BinaryOperator::GreaterThan,
                    Expression::Atom(ExpressionAtom::Literal(Value::Int(0), Position::new(0, 10))),
                    Position::new(0, 6)
                )),
                vec![],
                Position::new(0, 0)
            )
        );
    }
}
