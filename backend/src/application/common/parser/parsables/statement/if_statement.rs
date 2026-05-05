use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{change_err_msg, expect_token, get_pos, is_token}, parsable::Parsable, parsables::statement::Statement}, domain::{game::abstract_syntax_tree::{statement::{IfStatement, ElseBranch, ElseIfBranch}, expression::Expression}}};


impl Parsable for IfStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::If)
    }

    fn parse(ts: &mut TokenStream, source_code: &str) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(
            ts,
            TokenVariant::If,
            "Expected 'if' keyword at the beginning of if statement"
        );

        let condition = Expression::parse(ts, source_code).map_err(|err|
            change_err_msg!(err, "Expected expression in if statement condition")
        )?;

        expect_token!(
            ts,
            TokenVariant::OpenBrace,
            "Expected '{' after condition in if statement"
        );

        let mut then_branch = Vec::new();
        while !is_token!(ts, TokenVariant::CloseBrace) {
            then_branch.push(Statement::parse(ts, source_code)?);
        }

        expect_token!(
            ts,
            TokenVariant::CloseBrace,
            "Expected '}' at the end of then branch in if statement"
        );

        let mut else_if_branches = Vec::new();
        let mut else_branch = None;
        while is_token!(ts, TokenVariant::Else) {
            let else_pos = get_pos!(ts);
            ts.next(); // consume 'else' token

            if is_token!(ts, TokenVariant::If) {
                ts.next(); // consume 'if' token

                let else_if_condition = Expression::parse(ts, source_code).map_err(|err|
                    change_err_msg!(err, "Expected expression in else if branch condition")
                )?;

                expect_token!(
                    ts,
                    TokenVariant::OpenBrace,
                    "Expected '{' after condition in else if branch"
                );

                let mut else_if_then_branch = Vec::new();
                while !is_token!(ts, TokenVariant::CloseBrace) {
                    else_if_then_branch.push(Statement::parse(ts, source_code)?);
                }

                expect_token!(
                    ts,
                    TokenVariant::CloseBrace,
                    "Expected '}' at the end of then branch in else if branch"
                );

                else_if_branches.push(ElseIfBranch::new(else_if_condition, else_if_then_branch, else_pos));
            } else {
                expect_token!(
                    ts,
                    TokenVariant::OpenBrace,
                    "Expected '{' after 'else' keyword in else branch"
                );

                let mut else_then_branch = Vec::new();
                while !is_token!(ts, TokenVariant::CloseBrace) {
                    else_then_branch.push(Statement::parse(ts, source_code)?);
                }

                expect_token!(
                    ts,
                    TokenVariant::CloseBrace,
                    "Expected '}' at the end of else branch"
                );

                    else_branch = Some(ElseBranch::new(else_then_branch, else_pos));
            }
        }

        Ok(IfStatement::new(condition, then_branch, else_if_branches, else_branch, pos))
    }
}

#[cfg(test)]
mod tests {

    use crate::{application::common::parser::macros::test_token_stream, domain::{common::position::Position, game::{abstract_syntax_tree::{expression::{atom::ExpressionAtom, binary::{BinaryExpression, BinaryOperator}, unary::{UnaryExpression, UnaryOperator}}, statement::AssignmentStatement}, value_objects::data::Value}}};
    use super::*;

    #[test]
    fn test_plain_if_statement() {
        let (mut ts, source_code) = test_token_stream!("if x > 0.0 { y = 1.0; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 3))),
                    BinaryOperator::GreaterThan,
                    Expression::Atom(ExpressionAtom::Literal(Value::Float(0.0), Position::new(0, 7))),
                    Position::new(0, 3)
                )),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "y".to_string(),
                    Expression::Atom(ExpressionAtom::Literal(Value::Float(1.0), Position::new(0, 17))),
                    Position::new(0, 13)
                ))],
                vec![],
                None,
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_empty_if_statement() {
        let (mut ts, source_code) = test_token_stream!("if true { }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 3))),
                vec![],
                vec![],
                None,
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_if_else_statement() {
        let (mut ts, source_code) = test_token_stream!("if x > 0.0 { y = 1.0; } else { y = -1.0; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::Binary(BinaryExpression::new(
                    Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 3))),
                    BinaryOperator::GreaterThan,
                    Expression::Atom(ExpressionAtom::Literal(Value::Float(0.0), Position::new(0, 7))),
                    Position::new(0, 3)
                )),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "y".to_string(),
                    Expression::Atom(ExpressionAtom::Literal(Value::Float(1.0), Position::new(0, 17))),
                    Position::new(0, 13)
                ))],
                vec![],
                Some(ElseBranch::new(
                    vec![Statement::Assignment(AssignmentStatement::new(
                        "y".to_string(),
                        Expression::Unary(UnaryExpression::new(UnaryOperator::Negation, Expression::Atom(ExpressionAtom::Literal(Value::Float(1.0), Position::new(0, 36))), Position::new(0, 35))),
                        Position::new(0, 31)
                    ))],
                    Position::new(0, 24)
                )),
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_if_else_if_statement() {
        let (mut ts, source_code) = test_token_stream!("if false { } else if true { x = 42; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(false), Position::new(0, 3))),
                vec![],
                vec![ElseIfBranch::new(
                    Expression::Atom(ExpressionAtom::Literal(Value::Bool(true), Position::new(0, 21))),
                    vec![Statement::Assignment(AssignmentStatement::new(
                        "x".to_string(),
                        Expression::Atom(ExpressionAtom::Literal(Value::Int(42), Position::new(0, 32))),
                        Position::new(0, 28)
                    ))],
                    Position::new(0, 13)
                )],
                None,
                Position::new(0, 0)
            )
        );
    }

    #[test]
    fn test_if_else_if_else_statement() {
        let (mut ts, source_code) = test_token_stream!("if false { } else if x { } else { y = -1.0; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(false), Position::new(0, 3))),
                vec![],
                vec![ElseIfBranch::new(
                    Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 21))),
                    vec![],
                    Position::new(0, 13)
                )],
                Some(ElseBranch::new(
                    vec![Statement::Assignment(AssignmentStatement::new(
                        "y".to_string(),
                        Expression::Unary(UnaryExpression::new(UnaryOperator::Negation, Expression::Atom(ExpressionAtom::Literal(Value::Float(1.0), Position::new(0, 39))), Position::new(0, 38))),
                        Position::new(0, 34)
                    ))],
                    Position::new(0, 27)
                )),
                Position::new(0, 0),
            )
        );
    }

    #[test]
    fn test_multiple_else_if_branches() {
        let (mut ts, source_code) = test_token_stream!("if false { } else if x { } else if y { } else { z = 42; }");

        let if_stmt = IfStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            if_stmt,
            IfStatement::new(
                Expression::Atom(ExpressionAtom::Literal(Value::Bool(false), Position::new(0, 3))),
                vec![],
                vec![
                    ElseIfBranch::new(Expression::Atom(ExpressionAtom::Variable("x".to_string(), Position::new(0, 21))), vec![], Position::new(0, 13)),
                    ElseIfBranch::new(Expression::Atom(ExpressionAtom::Variable("y".to_string(), Position::new(0, 35))), vec![], Position::new(0, 27))
                ],
                Some(ElseBranch::new(
                    vec![Statement::Assignment(AssignmentStatement::new(
                        "z".to_string(),
                        Expression::Atom(ExpressionAtom::Literal(Value::Int(42), Position::new(0, 52))),
                        Position::new(0, 48)
                    ))],
                    Position::new(0, 41)
                )),
                Position::new(0, 0),
            )
        );
    }
}
