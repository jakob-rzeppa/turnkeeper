//! # If Statement
//!
//! AST node representing an `if` conditional statement in a plugin script.

use super::super::Expression;
use super::Statement;

/// An `if` statement with an optional `else` branch.
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub then_body: Vec<Box<Statement>>,
    pub else_body: Option<Vec<Box<Statement>>>,
}
