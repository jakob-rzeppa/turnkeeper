use crate::domain::{
    common::position::Position,
    game::value_objects::{
        execution_trigger::ExecutionTrigger, parameter::Parameter, visibility::ActionVisibility,
    },
};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ActionMetadataProjection {
    pub name: String,

    pub parameters: Vec<Parameter>,
    pub execution_triggers: Vec<ExecutionTrigger>,

    pub visibility: ActionVisibility,

    pub source_code: String,
    pub pos: Position,
}
