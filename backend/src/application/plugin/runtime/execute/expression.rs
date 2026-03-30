use crate::application::plugin::{
    parser::abstract_syntax_tree::{
        Positioned,
        expression::{
            Expression,
            atom::ExpressionAtom,
            binary::{BinaryExpression, BinaryOperator},
            unary::{UnaryExpression, UnaryOperator},
        },
    },
    runtime::{
        RuntimeEnvironment,
        error::RuntimeError,
        memory::{identifier::Identifier, values::VariableValue},
    },
};

impl RuntimeEnvironment {
    pub fn evaluate_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<VariableValue, RuntimeError> {
        match expr {
            Expression::Atom(atom) => self.evaluate_atom(atom),
            Expression::Unary(unary_expr) => self.evaluate_unary(unary_expr),
            Expression::Binary(binary_expr) => self.evaluate_binary(binary_expr),
        }
    }

    fn evaluate_atom(&mut self, atom: &ExpressionAtom) -> Result<VariableValue, RuntimeError> {
        match atom {
            ExpressionAtom::Literal(literal) => Ok(VariableValue::from(literal.value())),
            ExpressionAtom::Variable(var) => self
                .memory_manager
                .get_variable(&Identifier::from(var.identifier()))
                .map_err(|err| RuntimeError::VariableNotFound {
                    identifier: Identifier::from(var.identifier()),
                    pos: var.position(),
                })
                .cloned(),
            ExpressionAtom::FunctionCall(function_call) => unimplemented!(),
        }
    }

    fn evaluate_unary(
        &mut self,
        unary_expr: &UnaryExpression,
    ) -> Result<VariableValue, RuntimeError> {
        let operand_value = self.evaluate_expression(unary_expr.operand())?;

        match (unary_expr.operator(), &operand_value) {
            (UnaryOperator::Negation, VariableValue::Int(value)) => Ok(VariableValue::Int(-value)),
            (UnaryOperator::Negation, VariableValue::Float(value)) => {
                Ok(VariableValue::Float(-value))
            }
            (UnaryOperator::LogicalNot, VariableValue::Bool(value)) => {
                Ok(VariableValue::Bool(!value))
            }
            _ => Err(RuntimeError::UndefinedUnaryOperation {
                operator: format!("{}", unary_expr.operator()),
                operand: operand_value,
                pos: unary_expr.position(),
            }),
        }
    }

    fn evaluate_binary(
        &mut self,
        binary_expr: &BinaryExpression,
    ) -> Result<VariableValue, RuntimeError> {
        let left_value = self.evaluate_expression(binary_expr.left())?;
        let right_value = self.evaluate_expression(binary_expr.right())?;

        match (
            left_value.clone(),
            binary_expr.operator(),
            right_value.clone(),
        ) {
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
                        pos: binary_expr.position(),
                    });
                }
                Ok(VariableValue::Int(left / right))
            }
            (VariableValue::Float(left), BinaryOperator::Division, VariableValue::Float(right)) => {
                if right == 0.0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: binary_expr.position(),
                    });
                }
                Ok(VariableValue::Float(left / right))
            }

            // Modulo
            (VariableValue::Int(left), BinaryOperator::Modulo, VariableValue::Int(right)) => {
                if right == 0 {
                    return Err(RuntimeError::DivisionByZero {
                        pos: binary_expr.position(),
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
                operator: format!("{}", binary_expr.operator()),
                right: right_value,
                pos: binary_expr.position(),
            }),
        }
    }
}
