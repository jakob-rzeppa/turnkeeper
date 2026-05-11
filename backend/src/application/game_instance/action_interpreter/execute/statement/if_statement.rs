use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::{
        common::position::Positioned,
        game::{
            abstract_syntax_tree::statement::if_statement::IfStatement,
            value_objects::data::Value,
        },
    },
};

impl Executable<()> for IfStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let condition = self.condition();
        let condition_value = condition.execute(env).await?;

        match condition_value {
            Value::Bool(true) => {
                env.push_scope();
                for stmt in self.then_statements() {
                    match Box::pin(stmt.execute(env)).await {
                        Ok(()) => {}
                        Err(err) => {
                            env.pop_scope();
                            return Err(err);
                        }
                    }
                }
                env.pop_scope();
            }
            Value::Bool(false) => {
                for else_if in self.else_if_branches() {
                    let else_if_condition = else_if.condition();
                    let else_if_condition_value = else_if_condition.execute(env).await?;
                    if let Value::Bool(true) = else_if_condition_value {
                        env.push_scope();
                        for stmt in else_if.then_statements() {
                            match Box::pin(stmt.execute(env)).await {
                                Ok(()) => {}
                                Err(err) => {
                                    env.pop_scope();
                                    return Err(err);
                                }
                            }
                        }
                        env.pop_scope();
                        return Ok(());
                    } else if !matches!(else_if_condition_value, Value::Bool(_)) {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "boolean in else-if condition".to_string(),
                            found: else_if_condition_value,
                            pos: else_if_condition.position(),
                        });
                    }
                }

                if let Some(else_branch) = self.else_branch() {
                    env.push_scope();
                    for stmt in else_branch.then_statements() {
                        match Box::pin(stmt.execute(env)).await {
                            Ok(()) => {}
                            Err(err) => {
                                env.pop_scope();
                                return Err(err);
                            }
                        }
                    }
                    env.pop_scope();
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
