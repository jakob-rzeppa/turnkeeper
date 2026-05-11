use std::collections::HashMap;

use crate::domain::game::{ entities::game_instance::GameInstance, value_objects::data::Value };

#[derive(Debug, Clone, serde::Serialize)]
pub struct RuntimeEnvironment {
    /// The game instance the action is performed on
    game_instance: GameInstance,
    /// Stack of variable scopes, with the last element being the current scope
    variables: Vec<HashMap<String, Value>>,
}

impl RuntimeEnvironment {
    pub fn new(game_instance: GameInstance, params: HashMap<String, Value>) -> Self {
        // Start with a action scope containing the parameters
        let mut variables = vec![HashMap::new()];
        variables[0].extend(params);

        Self {
            game_instance,
            variables,
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn declare_variable(&mut self, name: String, value: Value) -> bool {
        if let Some(scope) = self.variables.last_mut() {
            if scope.contains_key(&name) {
                return false; // Variable already declared in the current scope
            }

            scope.insert(name, value);
            true
        } else {
            unreachable!("There is always a global var scope")
        }
    }

    pub fn assign_variable(&mut self, name: String, value: Value) -> bool {
        for scope in self.variables.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return true; // Variable found and assigned
            }
        }

        false // Variable not found in any scope
    }

    pub fn push_scope(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.variables.pop();
    }
}
