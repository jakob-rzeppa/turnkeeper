use crate::application::plugin::{
    parser::abstract_syntax_tree::{Positioned, statement::while_loop::WhileLoopStatement},
    runtime::{
        RuntimeEnvironment, error::RuntimeError, execute::Executable, memory::values::VariableValue,
    },
};

impl Executable<()> for WhileLoopStatement {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let condition = self.condition();

        loop {
            let condition_value = condition.execute(env)?;

            match condition_value {
                VariableValue::Bool(true) => {
                    env.memory_manager.push_scope();
                    for stmt in self.body() {
                        match stmt.execute(env) {
                            Ok(()) => {}
                            Err(err) => {
                                env.memory_manager.pop_scope();
                                return Err(err);
                            }
                        }
                    }
                    env.memory_manager.pop_scope();
                }
                VariableValue::Bool(false) => break,
                _ => {
                    return Err(RuntimeError::TypeMismatch {
                        expected: "boolean value as while condition".to_string(),
                        found: condition_value,
                        pos: condition.position(),
                    });
                }
            }
        }
        Ok(())
    }
}
