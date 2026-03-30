use crate::application::game::plugin::{
    parser::abstract_syntax_tree::{Positioned, statement::if_statement::IfStatement},
    runtime::{RuntimeEnvironment, error::RuntimeError, memory::values::VariableValue},
};

impl RuntimeEnvironment {
    pub fn execute_if_statement(&mut self, stmt: &IfStatement) -> Result<(), RuntimeError> {
        let condition = stmt.condition();
        let condition_value = self.evaluate_expression(condition)?;

        match condition_value {
            VariableValue::Bool(true) => {
                self.memory_manager.push_scope();
                for stmt in stmt.then_statements() {
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
            VariableValue::Bool(false) => {
                for else_if in stmt.else_if_branches() {
                    let else_if_condition = else_if.condition();
                    let else_if_condition_value = self.evaluate_expression(else_if_condition)?;
                    if let VariableValue::Bool(true) = else_if_condition_value {
                        self.memory_manager.push_scope();
                        for stmt in else_if.then_statements() {
                            match self.execute_statement(stmt) {
                                Ok(()) => {}
                                Err(err) => {
                                    self.memory_manager.pop_scope();
                                    return Err(err);
                                }
                            }
                        }
                        self.memory_manager.pop_scope();
                        return Ok(());
                    } else if !matches!(else_if_condition_value, VariableValue::Bool(_)) {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "boolean in else-if condition".to_string(),
                            found: else_if_condition_value,
                            pos: else_if_condition.position(),
                        });
                    }
                }

                if let Some(else_branch) = stmt.else_branch() {
                    self.memory_manager.push_scope();
                    for stmt in else_branch.then_statements() {
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
            }
            _ => {
                return Err(RuntimeError::TypeMismatch {
                    expected: "boolean in if condition".to_string(),
                    found: condition_value,
                    pos: condition.position(),
                });
            }
        }
        Ok(())
    }
}
