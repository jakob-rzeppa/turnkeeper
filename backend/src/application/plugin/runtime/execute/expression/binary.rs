use crate::application::plugin::{
    parser::abstract_syntax_tree::{
        Positioned,
        expression::binary::{BinaryExpression, BinaryOperator},
    },
    runtime::{
        RuntimeEnvironment, error::RuntimeError, execute::Executable, memory::values::VariableValue,
    },
};

impl Executable<VariableValue> for BinaryExpression {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<VariableValue, RuntimeError> {
        let left_value = self.left().execute(env)?;
        let right_value = self.right().execute(env)?;

        match (left_value.clone(), self.operator(), right_value.clone()) {
            // Addition
            (VariableValue::Int(left), BinaryOperator::Addition, VariableValue::Int(right)) => {
                Ok(VariableValue::Int(left + right))
            }
            (VariableValue::Float(left), BinaryOperator::Addition, VariableValue::Float(right)) => {
                Ok(VariableValue::Float(left + right))
            }
            (
                VariableValue::String(left),
                BinaryOperator::Addition,
                VariableValue::String(right),
            ) => Ok(VariableValue::String(left + &right)),

            // Subtraction
            (VariableValue::Int(left), BinaryOperator::Subtraction, VariableValue::Int(right)) => {
                Ok(VariableValue::Int(left - right))
            }
            (
                VariableValue::Float(left),
                BinaryOperator::Subtraction,
                VariableValue::Float(right),
            ) => Ok(VariableValue::Float(left - right)),

            // Multiplication
            (
                VariableValue::Int(left),
                BinaryOperator::Multiplication,
                VariableValue::Int(right),
            ) => Ok(VariableValue::Int(left * right)),
            (
                VariableValue::Float(left),
                BinaryOperator::Multiplication,
                VariableValue::Float(right),
            ) => Ok(VariableValue::Float(left * right)),

            // Division
            (VariableValue::Int(left), BinaryOperator::Division, VariableValue::Int(right)) => {
                if right == 0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: self.position(),
                    });
                }
                Ok(VariableValue::Int(left / right))
            }
            (VariableValue::Float(left), BinaryOperator::Division, VariableValue::Float(right)) => {
                if right == 0.0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: self.position(),
                    });
                }
                Ok(VariableValue::Float(left / right))
            }

            // Modulo
            (VariableValue::Int(left), BinaryOperator::Modulo, VariableValue::Int(right)) => {
                if right == 0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: self.position(),
                    });
                }
                Ok(VariableValue::Int(left % right))
            }

            // Power
            (VariableValue::Int(left), BinaryOperator::Power, VariableValue::Int(right)) => {
                Ok(VariableValue::Int(left.pow(right as u32)))
            }
            (VariableValue::Float(left), BinaryOperator::Power, VariableValue::Float(right)) => {
                Ok(VariableValue::Float(left.powf(right)))
            }
            (VariableValue::Int(left), BinaryOperator::Power, VariableValue::Float(right)) => {
                Ok(VariableValue::Float((left as f64).powf(right)))
            }
            (VariableValue::Float(left), BinaryOperator::Power, VariableValue::Int(right)) => {
                Ok(VariableValue::Float(left.powi(right as i32)))
            }

            // Logical AND
            (VariableValue::Bool(left), BinaryOperator::LogicalAnd, VariableValue::Bool(right)) => {
                Ok(VariableValue::Bool(left && right))
            }
            // Logical OR
            (VariableValue::Bool(left), BinaryOperator::LogicalOr, VariableValue::Bool(right)) => {
                Ok(VariableValue::Bool(left || right))
            }

            // Comparison operators
            (VariableValue::Int(left), BinaryOperator::Equal, VariableValue::Int(right)) => {
                Ok(VariableValue::Bool(left == right))
            }
            (VariableValue::Float(left), BinaryOperator::Equal, VariableValue::Float(right)) => {
                Ok(VariableValue::Bool(left == right))
            }
            (VariableValue::String(left), BinaryOperator::Equal, VariableValue::String(right)) => {
                Ok(VariableValue::Bool(left == right))
            }
            (VariableValue::Bool(left), BinaryOperator::Equal, VariableValue::Bool(right)) => {
                Ok(VariableValue::Bool(left == right))
            }
            (VariableValue::Int(left), BinaryOperator::NotEqual, VariableValue::Int(right)) => {
                Ok(VariableValue::Bool(left != right))
            }
            (VariableValue::Float(left), BinaryOperator::NotEqual, VariableValue::Float(right)) => {
                Ok(VariableValue::Bool(left != right))
            }
            (
                VariableValue::String(left),
                BinaryOperator::NotEqual,
                VariableValue::String(right),
            ) => Ok(VariableValue::Bool(left != right)),
            (VariableValue::Bool(left), BinaryOperator::NotEqual, VariableValue::Bool(right)) => {
                Ok(VariableValue::Bool(left != right))
            }
            (VariableValue::Int(left), BinaryOperator::LessThan, VariableValue::Int(right)) => {
                Ok(VariableValue::Bool(left < right))
            }
            (VariableValue::Float(left), BinaryOperator::LessThan, VariableValue::Float(right)) => {
                Ok(VariableValue::Bool(left < right))
            }
            (
                VariableValue::Int(left),
                BinaryOperator::LessThanOrEqual,
                VariableValue::Int(right),
            ) => Ok(VariableValue::Bool(left <= right)),
            (
                VariableValue::Float(left),
                BinaryOperator::LessThanOrEqual,
                VariableValue::Float(right),
            ) => Ok(VariableValue::Bool(left <= right)),
            (VariableValue::Int(left), BinaryOperator::GreaterThan, VariableValue::Int(right)) => {
                Ok(VariableValue::Bool(left > right))
            }
            (
                VariableValue::Float(left),
                BinaryOperator::GreaterThan,
                VariableValue::Float(right),
            ) => Ok(VariableValue::Bool(left > right)),
            (
                VariableValue::Int(left),
                BinaryOperator::GreaterThanOrEqual,
                VariableValue::Int(right),
            ) => Ok(VariableValue::Bool(left >= right)),
            (
                VariableValue::Float(left),
                BinaryOperator::GreaterThanOrEqual,
                VariableValue::Float(right),
            ) => Ok(VariableValue::Bool(left >= right)),
            _ => Err(RuntimeError::UndefinedBinaryOperation {
                left: left_value,
                operator: format!("{}", self.operator()),
                right: right_value,
                pos: self.position(),
            }),
        }
    }
}
