use crate::domain::{common::position::Position, game::value_objects::data::Value};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExpressionAtom {
    Literal(Value, Position),
    Variable(String, Position),
}