use crate::domain::{
    common::position::Position,
    game::value_objects::{
        data::VariableType, execution_trigger::ExecutionTrigger, visibility::ActionVisibility,
    },
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Action {
    name: String,

    // A Action can either have parameters or execution triggers. This is because a triggered Action is executed automatically and therefore cannot have parameters, while a non-triggered Action is executed manually and can have parameters.
    parameters: Vec<(String, VariableType)>, // Optional vec of (param_name, param_type)
    execution_triggers: Vec<ExecutionTrigger>, // Optional triggers for the action

    visibility: ActionVisibility,

    source_code: String,
    pos: Position,
}

impl Action {
    pub fn new(
        name: String,
        parameters: Vec<(String, VariableType)>,
        execution_triggers: Vec<ExecutionTrigger>,
        visibility: ActionVisibility,
        source_code: String,
        pos: Position,
    ) -> Self {
        if !parameters.is_empty() && !execution_triggers.is_empty() {
            panic!("An action cannot have both parameters and execution triggers");
        }

        Self {
            name,
            parameters,
            execution_triggers,
            visibility,
            source_code,
            pos,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn visibility(&self) -> &ActionVisibility {
        &self.visibility
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }
}
