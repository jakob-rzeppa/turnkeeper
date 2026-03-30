use std::collections::HashMap;

use crate::application::game::plugin::runtime::memory::{
    identifier::Identifier, values::VariableValue,
};

pub mod identifier;
pub mod values;

pub struct MemoryManager {
    /// Stack of variable scopes. Each scope is a HashMap of variable names to their values.
    variables: Vec<HashMap<Identifier, VariableValue>>,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            variables: vec![HashMap::new()], // Start with a global scope
        }
    }
}

impl MemoryManager {
    pub fn push_scope(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.variables.len() <= 1 {
            panic!("Cannot pop the global scope");
        }

        self.variables.pop();
    }

    fn current_scope(&mut self) -> &mut HashMap<Identifier, VariableValue> {
        self.variables
            .last_mut()
            .expect("there should always be a active scope")
    }

    pub fn declare_variable(
        &mut self,
        name: Identifier,
        value: VariableValue,
    ) -> Result<(), String> {
        // Check if the variable already exists in the current scope
        // If the variable already exists in a higher scope, we allow it to be shadowed, but if it exists in the current scope, it's an error
        if self.current_scope().contains_key(&name) {
            return Err(format!(
                "Variable '{}' already declared in the current scope",
                name
            ));
        }

        self.current_scope().insert(name, value);
        Ok(())
    }

    pub fn assign_variable(
        &mut self,
        name: Identifier,
        value: VariableValue,
    ) -> Result<(), String> {
        // Look for the variable first in the current scope and then in outer scopes
        for scope in self.variables.iter_mut().rev() {
            if scope.contains_key(&name) {
                if !scope.get(&name).unwrap().is_type(&value) {
                    return Err(format!(
                        "Type mismatch: cannot assign value of type {:?} to variable '{}'",
                        value, name
                    ));
                }

                scope.insert(name, value);
                return Ok(());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }

    pub fn get_variable(&self, name: &Identifier) -> Result<&VariableValue, String> {
        // Look for the variable first in the current scope and then in outer scopes
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value);
            }
        }
        Err(format!("Variable '{}' not found", name))
    }
}
