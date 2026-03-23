use crate::application::game::plugin::{parser::abstract_syntax_tree::statement::IfStatement, runtime::{RuntimeEnvironment, memory::VariableValue}};


impl RuntimeEnvironment {
    pub fn execute_if_stmt(&mut self, stmt: &IfStatement) -> Result<(), String> {
        let condition_value = self.evaluate_expression(&stmt.condition)?;

        match condition_value {
            VariableValue::Bool(true) => {
                for stmt in &stmt.then.0 {
                    self.execute_stmt(stmt)?;
                }
            },
            VariableValue::Bool(false) => {
                // Do nothing for false condition
            },
            _ => return Err("If condition expression must evaluate to a boolean".to_string()),
        }
        Ok(())
    }
}