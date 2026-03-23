use crate::application::game::plugin::{parser::abstract_syntax_tree::statement::ExprStatement, runtime::RuntimeEnvironment};


impl RuntimeEnvironment {
    pub fn execute_expression_stmt(&mut self, expression_stmt: &ExprStatement) -> Result<(), String> {
        self.evaluate_expression(&expression_stmt.0)?;
        Ok(())
    }
}