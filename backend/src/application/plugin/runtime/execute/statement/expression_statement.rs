use backend_derive::execute_debug;

use crate::application::plugin::{
    parser::abstract_syntax_tree::statement::expression::ExpressionStatement,
    runtime::{RuntimeEnvironment, error::RuntimeError, execute::Executable},
};

impl Executable<()> for ExpressionStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        self.expression().execute(env).await?;
        Ok(())
    }
}
