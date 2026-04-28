use crate::domain::{
    common::position::Position,
    game::{
        projections::action::ActionMetadataProjection,
        value_objects::{
            execution_trigger::ExecutionTrigger, parameter::Parameter, visibility::ActionVisibility,
        },
    },
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Action {
    name: String,

    // A Action can either have parameters or execution triggers. This is because a triggered Action is executed automatically and therefore cannot have parameters, while a non-triggered Action is executed manually and can have parameters.
    parameters: Vec<Parameter>, // Optional vec of parameters
    execution_triggers: Vec<ExecutionTrigger>, // Optional triggers for the action

    visibility: ActionVisibility,

    source_code: String,
    pos: Position,
}

impl Action {
    pub fn new(
        name: String,
        parameters: Vec<Parameter>,
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

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn execution_triggers(&self) -> &Vec<ExecutionTrigger> {
        &self.execution_triggers
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

    pub fn get_metadata_projection(&self) -> ActionMetadataProjection {
        ActionMetadataProjection {
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            execution_triggers: self.execution_triggers.clone(),
            visibility: self.visibility.clone(),
            source_code: self.source_code.clone(),
            pos: self.pos.clone(),
        }
    }
}
