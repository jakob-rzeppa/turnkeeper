use serde::Serialize;

use crate::application::plugin::runtime::memory::{identifier::Identifier, values::VariableValue};

#[derive(Debug, Serialize)]
pub struct DebugState {
    pub variables: Vec<(Identifier, VariableValue)>,
}
