use crate::application::plugin::{
    parser::abstract_syntax_tree::{Positioned, statement::if_statement::IfStatement},
    runtime::{
        RuntimeEnvironment, error::RuntimeError, execute::Executable, memory::values::VariableValue,
    },
};

impl Executable<()> for IfStatement {
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let condition = self.condition();
        let condition_value = condition.execute(env).await?;

        match condition_value {
            VariableValue::Bool(true) => {
                env.memory_manager.push_scope();
                for stmt in self.then_statements() {
                    match Box::pin(stmt.execute(env)).await {
                        Ok(()) => {}
                        Err(err) => {
                            env.memory_manager.pop_scope();
                            return Err(err);
                        }
                    }
                }
                env.memory_manager.pop_scope();
            }
            VariableValue::Bool(false) => {
                for else_if in self.else_if_branches() {
                    let else_if_condition = else_if.condition();
                    let else_if_condition_value = else_if_condition.execute(env).await?;
                    if let VariableValue::Bool(true) = else_if_condition_value {
                        env.memory_manager.push_scope();
                        for stmt in else_if.then_statements() {
                            match Box::pin(stmt.execute(env)).await {
                                Ok(()) => {}
                                Err(err) => {
                                    env.memory_manager.pop_scope();
                                    return Err(err);
                                }
                            }
                        }
                        env.memory_manager.pop_scope();
                        return Ok(());
                    } else if !matches!(else_if_condition_value, VariableValue::Bool(_)) {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "boolean in else-if condition".to_string(),
                            found: else_if_condition_value,
                            pos: else_if_condition.position(),
                        });
                    }
                }

                if let Some(else_branch) = self.else_branch() {
                    env.memory_manager.push_scope();
                    for stmt in else_branch.then_statements() {
                        match Box::pin(stmt.execute(env)).await {
                            Ok(()) => {}
                            Err(err) => {
                                env.memory_manager.pop_scope();
                                return Err(err);
                            }
                        }
                    }
                    env.memory_manager.pop_scope();
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
