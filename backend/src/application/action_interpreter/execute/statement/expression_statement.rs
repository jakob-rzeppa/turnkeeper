use backend_derive::execute_debug;

use crate::{
    application::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::abstract_syntax_tree::statement::expression_statement::ExpressionStatement,
};

impl Executable<()> for ExpressionStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        _ = self.expression().execute(env).await?;
        Ok(())
    }
}
