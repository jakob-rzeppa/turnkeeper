
use crate::{application::common::parser::lexer::token::{Token, TokenVariant}, domain::game::abstract_syntax_tree::expression::binary::BinaryOperator};


// Since BinaryExpression is not directly parsable (it requires operator precedence parsing), we don't implement Parsable for it. 
// Instead, we'll implement the parsing logic in the Expression, where we can handle operator precedence and associativity correctly.

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
            TokenVariant::LessThan => Some(BinaryOperator::LessThan),
            TokenVariant::LessThanEqual => Some(BinaryOperator::LessThanOrEqual),
            TokenVariant::GreaterThan => Some(BinaryOperator::GreaterThan),
            TokenVariant::GreaterThanEqual => Some(BinaryOperator::GreaterThanOrEqual),
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