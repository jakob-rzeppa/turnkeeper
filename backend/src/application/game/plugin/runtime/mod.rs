use crate::application::game::plugin::{
    common::Position,
    parser::parse_source_code,
    runtime::memory::{MemoryManager, identifier::Identifier, values::VariableValue},
};

mod execute;
mod memory;

pub struct RuntimeEnvironment {
    memory_manager: MemoryManager,
}

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

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::default(),
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), String> {
        let abstract_syntax_tree =
            parse_source_code(code).map_err(|err| err.context_message(code))?;

        for statement in abstract_syntax_tree.statements() {
            self.execute_statement(&statement)
                .map_err(|_| "".to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime() {
        let code = r#"
            let x: int = 42;
            x = x + 1;
            print(x);
        "#;

        let mut runtime = RuntimeEnvironment::new();
        let result = runtime.run(code);
        assert!(result.is_ok());
    }
}
