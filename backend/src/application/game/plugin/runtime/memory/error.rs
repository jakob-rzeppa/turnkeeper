use crate::application::game::plugin::runtime::memory::{
    identifier::Identifier, values::VariableValue,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryError {
    VariableNotFound(Identifier),
    VariableAlreadyDeclared(Identifier),
    TypeMismatch {
        expected: String,
        found: VariableValue,
    },
}
