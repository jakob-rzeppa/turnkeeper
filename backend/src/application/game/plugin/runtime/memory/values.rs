use crate::application::game::plugin::parser::abstract_syntax_tree::expression::atom::literal::LiteralValue;

#[derive(Clone, Debug, PartialEq)]
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
