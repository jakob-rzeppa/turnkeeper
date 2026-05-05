use std::fmt::Display;

use crate::{domain::{common::position::{Position, Positioned}, game::abstract_syntax_tree::expression::Expression}};


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BinaryExpression {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
    pos: Position,
}

impl BinaryExpression {
    pub fn new(
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
        pos: Position,
    ) -> Self {
        BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            pos,
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn operator(&self) -> &BinaryOperator {
        &self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

impl Positioned for BinaryExpression {
    fn position(&self) -> Position {
        self.pos
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Power,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOperator::Addition => "+",
            BinaryOperator::Subtraction => "-",
            BinaryOperator::Multiplication => "*",
            BinaryOperator::Division => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Power => "^",
            BinaryOperator::LogicalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
        };
        write!(f, "{}", op_str)
    }
}