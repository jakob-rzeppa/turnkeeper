use crate::application::plugin::{
    parser::abstract_syntax_tree::statement::expression::ExpressionStatement,
    runtime::{RuntimeEnvironment, error::RuntimeError, execute::Executable},
};

impl Executable<()> for ExpressionStatement {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        self.expression().execute(env)?;
        Ok(())
    }
}
