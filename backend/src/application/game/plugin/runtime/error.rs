use std::{error::Error, fmt::Display};

use crate::application::game::plugin::{
    common::Position, parser::abstract_syntax_tree::atom::identifier::Identifier,
    runtime::memory::values::VariableValue,
};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    VariableNotFound {
        identifier: Identifier,
        pos: Position,
    },
    TypeMismatch {
        expected: String,
        found: VariableValue,
        pos: Position,
    },
    UndefinedUnaryOperation {
        operator: String,
        operand: VariableValue,
        pos: Position,
    },
    UndefinedBinaryOperation {
        left: VariableValue,
        operator: String,
        right: VariableValue,
        pos: Position,
    },
    DivisionByZero {
        pos: Position,
    },
    Temp {
        message: String,
        pos: Position,
    },
}

impl RuntimeError {
    pub fn message(&self) -> String {
        match self {
            RuntimeError::VariableNotFound { identifier, .. } => {
                format!("Variable '{}' not found", identifier.name())
            }
            RuntimeError::TypeMismatch {
                expected, found, ..
            } => {
                format!("Type mismatch: expected {}, found {}", expected, found)
            }
            RuntimeError::UndefinedUnaryOperation {
                operator, operand, ..
            } => {
                format!(
                    "Undefined unary operation '{}' for operand {}",
                    operator, operand
                )
            }
            RuntimeError::UndefinedBinaryOperation {
                left,
                operator,
                right,
                ..
            } => {
                format!(
                    "Undefined binary operation '{} {} {}'",
                    left, operator, right
                )
            }
            RuntimeError::DivisionByZero { .. } => "Division by zero".to_string(),
            RuntimeError::Temp { message, .. } => message.clone(),
        }
    }

    pub fn context_message(&self, source_code: &str) -> String {
        let pos = match self {
            RuntimeError::VariableNotFound { pos, .. }
            | RuntimeError::TypeMismatch { pos, .. }
            | RuntimeError::UndefinedUnaryOperation { pos, .. }
            | RuntimeError::UndefinedBinaryOperation { pos, .. }
            | RuntimeError::DivisionByZero { pos, .. }
            | RuntimeError::Temp { pos, .. } => *pos,
        };

        let lines: Vec<&str> = source_code.lines().collect();

        let line_num = pos.line() as usize;
        let col_num = pos.column() as usize;

        let mut result = String::new();

        // Previous 2 lines
        let start_idx = line_num.saturating_sub(2);
        for i in start_idx..line_num {
            if i < lines.len() {
                result.push_str(lines[i]);
                result.push('\n');
            }
        }

        // Separator
        result.push_str(&"=".repeat(80));
        result.push('\n');

        // Line with error
        result.push_str(lines[line_num]);
        result.push('\n');

        // Indicator
        result.push_str(&" ".repeat(col_num));
        result.push_str("^");
        result.push('\n');

        // Error message
        result.push_str(&self.message());
        result.push('\n');

        // Separator
        result.push_str(&"=".repeat(80));
        result.push('\n');

        // Next 2 lines
        for i in (line_num + 1)..=(line_num + 2) {
            if i < lines.len() {
                result.push_str(lines[i]);
                result.push('\n');
            }
        }

        result
    }
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: {}", self.message())
    }
}
