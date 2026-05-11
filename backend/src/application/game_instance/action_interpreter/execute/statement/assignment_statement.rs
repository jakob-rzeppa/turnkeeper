use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::{
        common::position::Positioned,
        game::abstract_syntax_tree::statement::assignment::AssignmentStatement,
    },
};

impl Executable<()> for AssignmentStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let value = self.value().execute(env).await?;

        if env.assign_variable(self.name().clone(), value) == true {
            Ok(())
        } else {
            Err(RuntimeError::UndefinedVariable {
                name: self.name().clone(),
                pos: self.position(),
            })
        }
    }
}
