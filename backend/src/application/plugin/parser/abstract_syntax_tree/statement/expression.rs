use crate::application::plugin::{
    common::Position,
    lexer::token::TokenVariant,
    parser::{
        abstract_syntax_tree::{Parsable, Positioned, TokenStream, expression::Expression},
        error::ParsingError,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    expression: Expression,
    pos: Position,
}

impl ExpressionStatement {
    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl Parsable for ExpressionStatement {
    fn is_next(ts: &TokenStream) -> bool {
        Expression::is_next(ts)
    }

    fn parse(ts: &mut TokenStream) -> Result<Self, ParsingError> {
        let pos = get_pos!(ts);

        let expression = expect_parse!(ts, Expression, "expression in expression statement");

        expect_token!(
            ts,
            TokenVariant::Semicolon,
            "';' after expression statement"
        );

        Ok(ExpressionStatement { expression, pos })
    }
}

impl Positioned for ExpressionStatement {
    fn position(&self) -> Position {
        self.pos
    }
}

#[cfg(test)]
impl ExpressionStatement {
    pub fn new(expression: Expression, line: usize, column: usize) -> Self {
        Self {
            expression,
            pos: Position::new(line, column),
        }
    }
}
