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
            abstract_syntax_tree::expression::binary::{ BinaryExpression, BinaryOperator },
            value_objects::data::Value,
        },
    },
};

impl Executable<Value> for BinaryExpression {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<Value, RuntimeError> {
        let left_value: Value = self.left().execute(env).await?;
        let right_value: Value = self.right().execute(env).await?;

        match (left_value.clone(), self.operator(), right_value.clone()) {
            // Addition
            (Value::Int(left), BinaryOperator::Addition, Value::Int(right)) => {
                Ok(Value::Int(left + right))
            }
            (Value::Float(left), BinaryOperator::Addition, Value::Float(right)) => {
                Ok(Value::Float(left + right))
            }
            (Value::String(left), BinaryOperator::Addition, Value::String(right)) =>
                Ok(Value::String(left + &right)),

            // Subtraction
            (Value::Int(left), BinaryOperator::Subtraction, Value::Int(right)) => {
                Ok(Value::Int(left - right))
            }
            (Value::Float(left), BinaryOperator::Subtraction, Value::Float(right)) =>
                Ok(Value::Float(left - right)),

            // Multiplication
            (Value::Int(left), BinaryOperator::Multiplication, Value::Int(right)) =>
                Ok(Value::Int(left * right)),
            (Value::Float(left), BinaryOperator::Multiplication, Value::Float(right)) =>
                Ok(Value::Float(left * right)),

            // Division
            (Value::Int(left), BinaryOperator::Division, Value::Int(right)) => {
                if right == 0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: self.position(),
                    });
                }
                Ok(Value::Int(left / right))
            }
            (Value::Float(left), BinaryOperator::Division, Value::Float(right)) => {
                if right == 0.0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: self.position(),
                    });
                }
                Ok(Value::Float(left / right))
            }

            // Modulo
            (Value::Int(left), BinaryOperator::Modulo, Value::Int(right)) => {
                if right == 0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: self.position(),
                    });
                }
                Ok(Value::Int(left % right))
            }

            // Power
            (Value::Int(left), BinaryOperator::Power, Value::Int(right)) => {
                Ok(Value::Int(left.pow(right as u32)))
            }
            (Value::Float(left), BinaryOperator::Power, Value::Float(right)) => {
                Ok(Value::Float(left.powf(right)))
            }
            (Value::Int(left), BinaryOperator::Power, Value::Float(right)) => {
                Ok(Value::Float((left as f64).powf(right)))
            }
            (Value::Float(left), BinaryOperator::Power, Value::Int(right)) => {
                Ok(Value::Float(left.powi(right as i32)))
            }

            // Logical AND
            (Value::Bool(left), BinaryOperator::LogicalAnd, Value::Bool(right)) => {
                Ok(Value::Bool(left && right))
            }
            // Logical OR
            (Value::Bool(left), BinaryOperator::LogicalOr, Value::Bool(right)) => {
                Ok(Value::Bool(left || right))
            }

            // Comparison operators
            (Value::Int(left), BinaryOperator::Equal, Value::Int(right)) => {
                Ok(Value::Bool(left == right))
            }
            (Value::Float(left), BinaryOperator::Equal, Value::Float(right)) => {
                Ok(Value::Bool(left == right))
            }
            (Value::String(left), BinaryOperator::Equal, Value::String(right)) => {
                Ok(Value::Bool(left == right))
            }
            (Value::Bool(left), BinaryOperator::Equal, Value::Bool(right)) => {
                Ok(Value::Bool(left == right))
            }
            (Value::Int(left), BinaryOperator::NotEqual, Value::Int(right)) => {
                Ok(Value::Bool(left != right))
            }
            (Value::Float(left), BinaryOperator::NotEqual, Value::Float(right)) => {
                Ok(Value::Bool(left != right))
            }
            (Value::String(left), BinaryOperator::NotEqual, Value::String(right)) =>
                Ok(Value::Bool(left != right)),
            (Value::Bool(left), BinaryOperator::NotEqual, Value::Bool(right)) => {
                Ok(Value::Bool(left != right))
            }
            (Value::Int(left), BinaryOperator::LessThan, Value::Int(right)) => {
                Ok(Value::Bool(left < right))
            }
            (Value::Float(left), BinaryOperator::LessThan, Value::Float(right)) => {
                Ok(Value::Bool(left < right))
            }
            (Value::Int(left), BinaryOperator::LessThanOrEqual, Value::Int(right)) =>
                Ok(Value::Bool(left <= right)),
            (Value::Float(left), BinaryOperator::LessThanOrEqual, Value::Float(right)) =>
                Ok(Value::Bool(left <= right)),
            (Value::Int(left), BinaryOperator::GreaterThan, Value::Int(right)) => {
                Ok(Value::Bool(left > right))
            }
            (Value::Float(left), BinaryOperator::GreaterThan, Value::Float(right)) =>
                Ok(Value::Bool(left > right)),
            (Value::Int(left), BinaryOperator::GreaterThanOrEqual, Value::Int(right)) =>
                Ok(Value::Bool(left >= right)),
            (Value::Float(left), BinaryOperator::GreaterThanOrEqual, Value::Float(right)) =>
                Ok(Value::Bool(left >= right)),
            _ =>
                Err(RuntimeError::UndefinedBinaryOperation {
                    left: left_value,
                    operator: format!("{}", self.operator()),
                    right: right_value,
                    pos: self.position(),
                }),
        }
    }
}
