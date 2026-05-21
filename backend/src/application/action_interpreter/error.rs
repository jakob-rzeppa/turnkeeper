use crate::domain::{
    common::{ identifier::Id, position::Position },
    game::{
        error::GameInstanceError,
        value_objects::{ data::Value, visibility::ActionVisibility },
    },
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum ActionInterpreterError {
    #[error("Action not found: {0}")] ActionNotFound(String),
    #[error(
        "Permission denied for user: {user_id} with visibility: {visibility}"
    )] PermissionDenied {
        user_id: Id,
        visibility: ActionVisibility,
    },
    #[error("Missing parameter: {0}")] MissingParameter(String),
    #[error("Runtime error: {0}")] RuntimeError(#[from] RuntimeError),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum RuntimeError {
    #[error("Undefined unary operation: {operator} {operand} at {pos}")] UndefinedUnaryOperation {
        operator: String,
        operand: Value,
        pos: Position,
    },
    #[error(
        "Undefined binary operation: {operator} {left} {right} at {pos}"
    )] UndefinedBinaryOperation {
        operator: String,
        left: Value,
        right: Value,
        pos: Position,
    },
    #[error("Division by zero at {pos}")] DivisionByZero {
        pos: Position,
    },
    #[error("Variable not found: {name} at {pos}")] UndefinedVariable {
        name: String,
        pos: Position,
    },
    #[error("Type mismatch: expected {expected}, found {found} at {pos}")] TypeMismatch {
        expected: String,
        found: Value,
        pos: Position,
    },
    #[error("Variable already declared in this scope: {name} at {pos}")] VariableAlreadyDeclared {
        name: String,
        pos: Position,
    },
    #[error("Game instance error: {0}")] GameInstanceError(#[from] GameInstanceError),
}
