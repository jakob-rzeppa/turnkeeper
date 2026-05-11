use crate::domain::{
    common::position::Position,
    game::{ abstract_syntax_tree::expression::Expression, value_objects::data::Value },
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExpressionAtom {
    Literal(Value, Position),
    Variable(String, Position),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
        pos: Position,
    },
}
