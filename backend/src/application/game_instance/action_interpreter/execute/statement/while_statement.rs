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
            abstract_syntax_tree::statement::while_loop::WhileLoopStatement,
            value_objects::data::Value,
        },
    },
};

impl Executable<()> for WhileLoopStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let condition = self.condition();

        loop {
            let condition_value = condition.execute(env).await?;

            match condition_value {
                Value::Bool(true) => {
                    env.push_scope();
                    for stmt in self.body() {
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
                    break;
                }
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
