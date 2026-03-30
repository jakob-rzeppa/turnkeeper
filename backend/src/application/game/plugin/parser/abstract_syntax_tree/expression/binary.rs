use std::fmt::Display;

use crate::application::game::plugin::{
    common::Position,
    lexer::token::{Token, TokenVariant},
    parser::abstract_syntax_tree::{Positioned, expression::Expression},
};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
    pos: Position,
}

impl BinaryExpression {
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

#[derive(Debug, Clone, PartialEq)]
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

impl BinaryOperator {
    pub fn from_token(token: &Token) -> Option<Self> {
        match &token.variant {
            TokenVariant::Plus => Some(BinaryOperator::Addition),
            TokenVariant::Minus => Some(BinaryOperator::Subtraction),
            TokenVariant::Star => Some(BinaryOperator::Multiplication),
            TokenVariant::Slash => Some(BinaryOperator::Division),
            TokenVariant::Percent => Some(BinaryOperator::Modulo),
            TokenVariant::And => Some(BinaryOperator::LogicalAnd),
            TokenVariant::Or => Some(BinaryOperator::LogicalOr),
            TokenVariant::EqualEqual => Some(BinaryOperator::Equal),
            TokenVariant::NotEqual => Some(BinaryOperator::NotEqual),
            TokenVariant::Less => Some(BinaryOperator::LessThan),
            TokenVariant::LessEqual => Some(BinaryOperator::LessThanOrEqual),
            TokenVariant::Greater => Some(BinaryOperator::GreaterThan),
            TokenVariant::GreaterEqual => Some(BinaryOperator::GreaterThanOrEqual),
            TokenVariant::Caret => Some(BinaryOperator::Power),
            _ => None,
        }
    }

    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            BinaryOperator::LogicalOr => (40, 41),
            BinaryOperator::LogicalAnd => (50, 51),
            BinaryOperator::Equal | BinaryOperator::NotEqual => (60, 61),
            BinaryOperator::LessThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqual => (70, 71),
            BinaryOperator::Addition | BinaryOperator::Subtraction => (80, 81),
            BinaryOperator::Multiplication | BinaryOperator::Division | BinaryOperator::Modulo => {
                (90, 91)
            }
            BinaryOperator::Power => (101, 100),
        }
    }
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

/// Since BinaryExpression is not directly parsable (it requires operator precedence parsing), we don't implement Parsable for it. Instead, we'll implement the parsing logic in the Expression, where we can handle operator precedence and associativity correctly.
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
}
