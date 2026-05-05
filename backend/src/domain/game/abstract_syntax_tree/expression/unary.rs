use std::fmt::Display;

use crate::domain::{common::position::{Position, Positioned}, game::abstract_syntax_tree::expression::Expression};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnaryExpression {
    operator: UnaryOperator,
    operand: Box<Expression>,
    pos: Position,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UnaryOperator {
    Negation,
    LogicalNot,
}

impl UnaryExpression {
    pub fn new(operator: UnaryOperator, operand: Expression, pos: Position) -> Self {
        UnaryExpression {
            operator,
            operand: Box::new(operand),
            pos,
        }
    }

    pub fn operator(&self) -> &UnaryOperator {
        &self.operator
    }

    pub fn operand(&self) -> &Expression {
        &self.operand
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