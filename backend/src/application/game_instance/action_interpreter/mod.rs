use std::collections::HashMap;

use crate::{
    application::game_instance::action_interpreter::{
        error::ActionInterpreterError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::{
        common::identifier::Id,
        game::{
            entities::{ game_instance::GameInstance, weak::action::Action },
            value_objects::{ data::Value, visibility::ActionVisibility },
        },
    },
};

pub mod error;
pub mod runtime_env;
mod debug_env;
mod execute;

pub struct ActionExecutor {
    game_instance: GameInstance,
    action: Action,
    executing_user_id: Id,
}

impl ActionExecutor {
    pub fn new(
        game_instance: GameInstance,
        action_name: &str,
        executing_user_id: Id
    ) -> Result<Self, ActionInterpreterError> {
        let action = game_instance
            .actions()
            .iter()
            .find(|a| a.name() == action_name)
            .cloned()
            .ok_or_else(|| ActionInterpreterError::ActionNotFound(action_name.to_string()))?;

        match action.visibility() {
            ActionVisibility::Public => {}
            ActionVisibility::Private => {
                if game_instance.gm_user_id() != &executing_user_id {
                    return Err(ActionInterpreterError::PermissionDenied {
                        user_id: executing_user_id,
                        visibility: action.visibility().clone(),
                    });
                }
            }
            ActionVisibility::Hidden => {
                return Err(ActionInterpreterError::PermissionDenied {
                    user_id: executing_user_id,
                    visibility: action.visibility().clone(),
                });
            }
        }

        Ok(Self {
            game_instance,
            action,
            executing_user_id,
        })
    }

    pub async fn execute(
        self,
        params: HashMap<String, Value>
    ) -> Result<GameInstance, ActionInterpreterError> {
        for param in self.action.parameters() {
            if !params.contains_key(param.name()) {
                return Err(ActionInterpreterError::MissingParameter(param.name().to_string()));
            }
        }

        let mut runtime = RuntimeEnvironment::new(self.game_instance, params);

        for statement in self.action.execution_block() {
            statement.execute(&mut runtime).await?;
        }

        Ok(runtime.extract_updated_game_instance())
    }
}
