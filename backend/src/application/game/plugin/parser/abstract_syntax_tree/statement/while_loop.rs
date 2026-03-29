use crate::application::game::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::abstract_syntax_tree::{
        Parsable, ParsingError, TokenStream, expression::Expression, statement::Statement,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoopStatement {
    condition: Expression,
    body: Vec<Statement>,
    pos: Position,
}

impl Parsable for WhileLoopStatement {
    fn is_next(ts: &TokenStream) -> bool {
        is_token!(ts, TokenVariant::While)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        expect_token!(ts, TokenVariant::While, "Expected 'while' keyword");

        let condition = Expression::parse(ts)?;

        expect_token!(
            ts,
            TokenVariant::LeftBrace,
            "Expected '{' after while condition"
        );

        let mut body = Vec::new();
        while !is_token!(ts, TokenVariant::RightBrace) {
            body.push(Statement::parse(ts)?);
        }

        expect_token!(
            ts,
            TokenVariant::RightBrace,
            "Expected '}' to close while loop body"
        );

        Ok(WhileLoopStatement {
            condition,
            body,
            pos,
        })
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
    use crate::application::game::plugin::parser::abstract_syntax_tree::{
        expression::binary::BinaryOperator, statement::assignment::AssignmentStatement,
    };

    use super::*;

    #[test]
    fn test_parse_while_loop() {
        let mut ts = test_token_stream!(
            TokenVariant::While,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Less,
            TokenVariant::IntLiteral(10),
            TokenVariant::LeftBrace,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Assign,
            TokenVariant::IntLiteral(1),
            TokenVariant::Semicolon,
            TokenVariant::RightBrace
        );

        let while_loop = WhileLoopStatement::parse(&mut ts).unwrap();

        assert_eq!(
            while_loop,
            WhileLoopStatement::new_while_loop(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 1, 0),
                    BinaryOperator::LessThan,
                    Expression::new_atom_literal_int(10, 3, 0),
                    1,
                    0
                ),
                vec![Statement::Assignment(AssignmentStatement::new(
                    "x",
                    Expression::new_atom_literal_int(1, 7, 0),
                    5,
                    0
                ))],
                0,
                0
            )
        );
    }

    #[test]
    fn test_parse_while_loop_empty_body() {
        let mut ts = test_token_stream!(
            TokenVariant::While,
            TokenVariant::Identifier("x".to_string()),
            TokenVariant::Greater,
            TokenVariant::IntLiteral(0),
            TokenVariant::LeftBrace,
            TokenVariant::RightBrace
        );

        let while_loop = WhileLoopStatement::parse(&mut ts).unwrap();

        assert_eq!(
            while_loop,
            WhileLoopStatement::new_while_loop(
                Expression::new_binary(
                    Expression::new_atom_variable("x", 1, 0),
                    BinaryOperator::GreaterThan,
                    Expression::new_atom_literal_int(0, 3, 0),
                    1,
                    0
                ),
                vec![],
                0,
                0
            )
        );
    }
}
