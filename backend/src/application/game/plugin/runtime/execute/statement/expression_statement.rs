use crate::application::game::plugin::{parser::old_abstract_syntax_tree::statement::ExprStatement, runtime::RuntimeEnvironment};


impl RuntimeEnvironment {
    pub fn execute_expression_statement(&mut self, expression_stmt: &ExprStatement) -> Result<(), String> {
        self.evaluate_expression(&expression_stmt.0)?;
        Ok(())
    }
}