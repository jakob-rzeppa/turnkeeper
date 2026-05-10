use crate::{
    application::game_instance::action_interpreter::{
        debug_env::DebugEnvironment,
        error::ActionInterpreterError,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::entities::{ game_instance::GameInstance, weak::action::Action },
};

pub mod error;
pub mod runtime_env;
pub mod debug_env;

pub struct ActionExecutor {
    game_instance: GameInstance,
    action: Action,
}

impl ActionExecutor {
    pub fn new(
        game_instance: GameInstance,
        action_name: &str
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

pub trait Executable<R> {
    fn execute(
        &self,
        env: &mut RuntimeEnvironment
    ) -> impl std::future::Future<Output = Result<R, ActionInterpreterError>>;

    fn execute_debug(
        &self,
        env: &mut RuntimeEnvironment,
        debug_env: &mut DebugEnvironment
    ) -> impl std::future::Future<Output = Result<R, ActionInterpreterError>>;
}
