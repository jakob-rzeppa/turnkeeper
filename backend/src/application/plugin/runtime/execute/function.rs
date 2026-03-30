use crate::application::plugin::{
    parser::old_abstract_syntax_tree::expression::FunctionCall,
    runtime::{RuntimeEnvironment, memory::VariableValue},
};

impl RuntimeEnvironment {
    fn call_print(&mut self, arguments: Vec<VariableValue>) {
        println!("Print called with arguments: {:?}", arguments);
    }

    pub fn evaluate_function(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<VariableValue, String> {
        let function_identifier = function_call.identifier.to_string();

        match function_identifier.as_str() {
            "print" => {
                let arguments = function_call
                    .arguments
                    .iter()
                    .map(|arg| self.evaluate_expression(arg))
                    .collect::<Result<Vec<VariableValue>, String>>()?;
                self.call_print(arguments);
                Ok(VariableValue::None)
            }
            _ => Err(format!("Unknown function: {}", function_identifier)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::plugin::{
        common::Position,
        parser::old_abstract_syntax_tree::{
            expression::{Expr, ExprAtom, Literal},
            identifier::Identifier,
        },
    };

    use super::*;

    #[test]
    fn test_evaluate_function_print() {
        let mut runtime_env = RuntimeEnvironment::new();
        let function_call = FunctionCall {
            identifier: Identifier::new("print".to_string(), Position::new(0, 0)),
            arguments: vec![Expr::Atom(ExprAtom::Literal(Literal::String(
                "Hello, World!".to_string(),
            )))],
            catch_block: None,
        };
        let result = runtime_env.evaluate_function(&function_call);
        assert!(result.is_ok());
    }
}
