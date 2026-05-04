use crate::{
    application::game_instance::action_interpreter::error::ActionInterpreterError,
    domain::game::entities::{game_instance::GameInstance, weak::action::Action},
};

pub mod error;
pub struct ActionExecutor {
    game_instance: GameInstance,
    action: Action,
}

impl ActionExecutor {
    pub fn new(
        game_instance: GameInstance,
        action_name: &str,
    ) -> Result<Self, ActionInterpreterError> {
        let action = game_instance
            .actions()
            .iter()
            .find(|a| a.name() == action_name)
            .cloned()
            .ok_or_else(|| ActionInterpreterError::ActionNotFound(action_name.to_string()))?;

        Ok(Self {
            game_instance,
            action,
        })
    }
}
