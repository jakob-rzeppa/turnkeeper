use crate::application::game::plugin::{parser::{abstract_syntax_tree, parse_source_code}, runtime::memory::MemoryManager};

mod memory;
mod execute;

struct RuntimeEnvironment {
    memory_manager: MemoryManager,
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::default(),
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), String> {
        let abstract_syntax_tree = parse_source_code(code)?;

        for statement in abstract_syntax_tree.0 {
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
        let result = runtime.run(code);
        assert!(result.is_ok());
    }
}