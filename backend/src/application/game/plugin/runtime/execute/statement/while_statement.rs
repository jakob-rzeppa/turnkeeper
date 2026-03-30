use crate::application::game::plugin::{
    parser::abstract_syntax_tree::{Positioned, statement::while_loop::WhileLoopStatement},
    runtime::{RuntimeEnvironment, error::RuntimeError, memory::values::VariableValue},
};

impl RuntimeEnvironment {
    pub fn execute_while_statement(
        &mut self,
        stmt: &WhileLoopStatement,
    ) -> Result<(), RuntimeError> {
        let condition = stmt.condition();

        loop {
            let condition_value = self.evaluate_expression(condition)?;

            match condition_value {
                VariableValue::Bool(true) => {
                    self.memory_manager.push_scope();
                    for stmt in stmt.body() {
                        match self.execute_statement(stmt) {
                            Ok(()) => {}
                            Err(err) => {
                                self.memory_manager.pop_scope();
                                return Err(err);
                            }
                        }
                    }
                    self.memory_manager.pop_scope();
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
