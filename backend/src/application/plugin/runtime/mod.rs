use crate::application::plugin::{
    parser::{abstract_syntax_tree::root::Root, parse_source_code},
    runtime::{error::RuntimeError, memory::MemoryManager},
};

pub mod error;
mod execute;
mod memory;

pub struct RuntimeEnvironment {
    memory_manager: MemoryManager,
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::default(),
        }
    }

    pub fn run(&mut self, ast: &Root) -> Result<(), RuntimeError> {
        for statement in ast.statements() {
            self.execute_statement(&statement)?;
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
        let ast = parse_source_code(code).unwrap();
        let result = runtime.run(&ast);
        assert!(result.is_ok());
    }
}
