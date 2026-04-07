use crate::application::plugin::{
    parser::abstract_syntax_tree::{
        Positioned,
        expression::unary::{UnaryExpression, UnaryOperator},
    },
    runtime::{
        RuntimeEnvironment, error::RuntimeError, execute::Executable, memory::values::VariableValue,
    },
};
use backend_derive::execute_debug;

impl Executable<VariableValue> for UnaryExpression {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<VariableValue, RuntimeError> {
        let operand_value = self.operand().execute(env).await?;

        match (self.operator(), &operand_value) {
            (UnaryOperator::Negation, VariableValue::Int(value)) => Ok(VariableValue::Int(-value)),
            (UnaryOperator::Negation, VariableValue::Float(value)) => {
                Ok(VariableValue::Float(-value))
            }
            (UnaryOperator::LogicalNot, VariableValue::Bool(value)) => {
                Ok(VariableValue::Bool(!value))
            }
            _ => Err(RuntimeError::UndefinedUnaryOperation {
                operator: format!("{}", self.operator()),
                operand: operand_value,
                pos: self.position(),
            }),
        }
    }
}
