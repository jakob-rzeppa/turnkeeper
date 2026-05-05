use crate::domain::game::abstract_syntax_tree::expression::{atom::ExpressionAtom, binary::BinaryExpression, unary::UnaryExpression};

pub mod atom;
pub mod binary;
pub mod unary;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expression {
    Atom(ExpressionAtom),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}