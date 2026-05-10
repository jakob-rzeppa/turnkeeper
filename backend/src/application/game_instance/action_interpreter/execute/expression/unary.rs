use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::{
        common::position::Positioned,
        game::{
            abstract_syntax_tree::expression::unary::{ UnaryExpression, UnaryOperator },
            value_objects::data::Value,
        },
    },
};

impl Executable<Value> for UnaryExpression {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<Value, RuntimeError> {
        let operand_value = self.operand().execute(env).await?;

        match (self.operator(), &operand_value) {
            (UnaryOperator::Negation, Value::Int(value)) => Ok(Value::Int(-value)),
            (UnaryOperator::Negation, Value::Float(value)) => { Ok(Value::Float(-value)) }
            (UnaryOperator::LogicalNot, Value::Bool(value)) => { Ok(Value::Bool(!value)) }
            _ =>
                Err(RuntimeError::UndefinedUnaryOperation {
                    operator: format!("{}", self.operator()),
                    operand: operand_value,
                    pos: self.position(),
                }),
        }
    }
}
