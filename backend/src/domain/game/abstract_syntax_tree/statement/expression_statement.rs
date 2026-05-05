use crate::domain::{
    common::position::{Position, Positioned},
    game::abstract_syntax_tree::expression::Expression,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExpressionStatement {
    expression: Expression,
    pos: Position,
}

impl ExpressionStatement {
    pub fn new(expression: Expression, pos: Position) -> Self {
        ExpressionStatement { expression, pos }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl Positioned for ExpressionStatement {
    fn position(&self) -> Position {
        self.pos
    }
}
