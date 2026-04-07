use tokio::sync::{mpsc, watch};

use crate::application::plugin::{
    debugger::commands::DebugCommand,
    parser::abstract_syntax_tree::root::Root,
    runtime::{
        debug_mode::DebugMode, debug_state::DebugState, error::RuntimeError, execute::Executable,
        memory::MemoryManager,
    },
};

mod debug_env;
mod debug_mode;
mod debug_state;
pub mod error;
mod execute;
mod memory;

pub struct RuntimeEnvironment {
    memory_manager: MemoryManager,
    debug_mode: Option<DebugMode>,
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::default(),
            debug_mode: None,
        }
    }

    pub fn run(&mut self, ast: &Root) -> Result<(), RuntimeError> {
        for statement in ast.statements() {
            statement.execute(self)?;
        }

        Ok(())
    }

    pub fn run_debug_mode(
        &mut self,
        ast: &Root,
        breakpoints: Vec<usize>,
        command_receiver: mpsc::UnboundedReceiver<DebugCommand>,
        state_sender: watch::Sender<DebugState>,
    ) -> Result<(), RuntimeError> {
        self.debug_mode = Some(DebugMode::new(breakpoints, command_receiver, state_sender));

        self.run(ast)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::application::plugin::parser::parse_source_code;

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
