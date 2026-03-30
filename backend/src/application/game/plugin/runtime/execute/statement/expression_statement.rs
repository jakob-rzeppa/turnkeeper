use crate::application::game::plugin::{
    parser::abstract_syntax_tree::statement::expression::ExpressionStatement,
    runtime::{RuntimeEnvironment, error::RuntimeError},
};

impl RuntimeEnvironment {
    pub fn execute_expression_statement(
        &mut self,
        expression_stmt: &ExpressionStatement,
    ) -> Result<(), RuntimeError> {
        self.evaluate_expression(expression_stmt.expression())?;
        Ok(())
    }
}
