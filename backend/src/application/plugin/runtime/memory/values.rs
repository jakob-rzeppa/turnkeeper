use std::fmt::Display;

use serde::Serialize;

use crate::application::plugin::parser::abstract_syntax_tree::expression::atom::literal::LiteralValue;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum VariableValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl VariableValue {
    pub fn is_type(&self, other: &VariableValue) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl From<&LiteralValue> for VariableValue {
    fn from(literal: &LiteralValue) -> Self {
        match literal {
            LiteralValue::Int(val) => VariableValue::Int(*val),
            LiteralValue::Float(val) => VariableValue::Float(*val),
            LiteralValue::String(val) => VariableValue::String(val.clone()),
            LiteralValue::Bool(val) => VariableValue::Bool(*val),
        }
    }
}

impl Display for VariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableValue::Int(val) => write!(f, "(int) {}", val),
            VariableValue::Float(val) => write!(f, "(float) {}", val),
            VariableValue::String(val) => write!(f, "(string) \"{}\"", val),
            VariableValue::Bool(val) => write!(f, "(bool) {}", val),
            VariableValue::None => write!(f, "None"),
        }
    }
}
