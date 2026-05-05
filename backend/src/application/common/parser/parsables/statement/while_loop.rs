use crate::{application::common::parser::{error::ParsingError, lexer::{token::TokenVariant, token_stream::TokenStream}, macros::{expect_token, get_pos, is_token}, parsable::Parsable, parsables::{expression::Expression, statement::Statement}}, domain::common::position::{Position, Positioned}};



#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoopStatement {
    condition: Expression,
    body: Vec<Statement>,
    pos: Position,
}

impl WhileLoopStatement {
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn body(&self) -> &[Statement] {
        &self.body
    }
}

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

        Ok(WhileLoopStatement {
            condition,
            body,
            pos,
        })
    }
}

impl Positioned for WhileLoopStatement {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl WhileLoopStatement {
    pub fn new_while_loop(
        condition: Expression,
        body: Vec<Statement>,
        line: usize,
        column: usize,
    ) -> Self {
        WhileLoopStatement {
            condition,
            body,
            pos: Position::new(line, column),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::application::common::parser::{macros::test_token_stream, parsables::{expression::binary::BinaryOperator, statement::assignment::AssignmentStatement}};

use super::*;

    #[test]
    fn test_parse_while_loop() {
        let (mut ts, source_code) = test_token_stream!("while x < 10 { x = 1; }");

        let while_loop = WhileLoopStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            while_loop,
            WhileLoopStatement::new_while_loop(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 0, 6),
                    BinaryOperator::LessThan,
                    Expression::new_atom_literal_int(10, 0, 10),
                    0,
                    6
                ),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "x",
                    Expression::new_atom_literal_int(1, 0, 19),
                    0,
                    15
                ))],
                0,
                0
            )
        );
    }

    #[test]
    fn test_parse_while_loop_empty_body() {
        let (mut ts, source_code) = test_token_stream!("while x > 0 { }");

        let while_loop = WhileLoopStatement::parse(&mut ts, &source_code).unwrap();

        assert_eq!(
            while_loop,
            WhileLoopStatement::new_while_loop(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 0, 6),
                    BinaryOperator::GreaterThan,
                    Expression::new_atom_literal_int(0, 0, 10),
                    0,
                    6
                ),
                vec![],
                0,
                0
            )
        );
    }
}
