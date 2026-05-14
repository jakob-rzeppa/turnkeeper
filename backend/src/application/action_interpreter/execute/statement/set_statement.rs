use backend_derive::execute_debug;

use crate::{
    application::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::abstract_syntax_tree::statement::set_statement::SetStatement,
};

impl Executable<()> for SetStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let value = self.value().execute(env).await?;

        env.set_game_stat(self.stat(), value)?;

        Ok(())
    }
}
