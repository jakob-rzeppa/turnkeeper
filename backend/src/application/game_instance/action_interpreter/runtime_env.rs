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
    pub fn new(game_instance: GameInstance) -> Self {
        Self {
            game_instance,
            variables: vec![HashMap::new()], // Start with a global scope
        }
    }
}
