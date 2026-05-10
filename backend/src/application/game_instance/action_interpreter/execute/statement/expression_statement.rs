use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::abstract_syntax_tree::statement::ExpressionStatement,
};

impl Executable<()> for ExpressionStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        _ = self.expression().execute(env).await?;
        Ok(())
    }
}
