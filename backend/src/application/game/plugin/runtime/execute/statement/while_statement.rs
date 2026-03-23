use crate::application::game::plugin::{parser::abstract_syntax_tree::statement::WhileStatement, runtime::{RuntimeEnvironment, memory::VariableValue}};


impl RuntimeEnvironment {
    pub fn execute_while_statement(&mut self, stmt: &WhileStatement) -> Result<(), String> {
        loop {
            let condition_value = self.evaluate_expression(&stmt.condition)?;

            match condition_value {
                VariableValue::Bool(true) => {
                    for stmt in &stmt.body.0 {
                        self.execute_statement(stmt)?;
                    }
                },
                VariableValue::Bool(false) => break,
                _ => return Err("While condition expression must evaluate to a boolean".to_string()),
            }
        }
        Ok(())
    }
}