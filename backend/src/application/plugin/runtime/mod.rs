use crate::application::plugin::{
    parser::abstract_syntax_tree::root::Root,
    runtime::{
        debug::{DebugEnvironment, state::DebugState},
        error::RuntimeError,
        execute::Executable,
        memory::MemoryManager,
    },
};

pub mod debug;
pub mod error;
mod execute;
pub mod memory;

pub struct RuntimeEnvironment {
    memory_manager: MemoryManager,
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::default(),
        }
    }

    pub async fn run(&mut self, ast: &Root) -> Result<(), RuntimeError> {
        for statement in ast.statements() {
            statement.execute(self).await?;
        }

        Ok(())
    }

    pub async fn run_debug_mode(
        &mut self,
        ast: &Root,
        debug_env: &mut DebugEnvironment,
    ) -> Result<(), RuntimeError> {
        for statement in ast.statements() {
            statement.execute_debug(self, debug_env).await?;
        }
        Ok(())
    }

    pub fn get_debug_state(&self) -> DebugState {
        DebugState {
            variables: self.memory_manager.get_all_variables(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::plugin::parser::parse_source_code;

    use super::*;

    #[tokio::test]
    async fn test_runtime() {
        let code = r#"
            let x: int = 42;
            x = x + 1;
            print(x);
        "#;

        let mut runtime = RuntimeEnvironment::new();
        let ast = parse_source_code(code).unwrap();
        let result = runtime.run(&ast).await;
        assert!(result.is_ok());
    }
}
