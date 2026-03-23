use crate::application::game::plugin::{parser::abstract_syntax_tree::expression::{BinaryOperator, Expr, ExprAtom, Literal, UnaryOperator}, runtime::{RuntimeEnvironment, memory::VariableValue}};


impl RuntimeEnvironment {
    fn evaluate_atom(&mut self, atom: &ExprAtom) -> Result<VariableValue, String> {
        match atom {
            ExprAtom::Literal(literal) => {
                match literal {
                    Literal::Int(value) => Ok(VariableValue::Int(*value)),
                    Literal::Float(value) => Ok(VariableValue::Float(*value)),
                    Literal::String(value) => Ok(VariableValue::String(value.clone())),
                    Literal::Bool(value) => Ok(VariableValue::Bool(*value)),
                }
            },
            ExprAtom::Identifier(name) => self.memory_manager.get_variable(name.as_str()).cloned(),
            ExprAtom::FunctionCall(function_call) => {
                match self.evaluate_function(function_call) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(format!("Error evaluating function call '{}': {}", function_call.identifier.as_str(), err)),
                }
            }
        }
    }

    fn evaluate_unary_operation(&mut self, operator: &UnaryOperator, operand: &Expr) -> Result<VariableValue, String> {
        let operand_value = self.evaluate_expression(operand)?;

        match operator {
            UnaryOperator::Neg => {
                match operand_value {
                    VariableValue::Int(value) => Ok(VariableValue::Int(-value)),
                    VariableValue::Float(value) => Ok(VariableValue::Float(-value)),
                    _ => Err("Negation operator can only be applied to int or float".to_string()),
                }
            },
            UnaryOperator::Not => {
                match operand_value {
                    VariableValue::Bool(value) => Ok(VariableValue::Bool(!value)),
                    _ => Err("Not operator can only be applied to bool".to_string()),
                }
            },
        }
    }

    fn evaluate_binary_operation(&mut self, left: &Expr, operator: &BinaryOperator, right: &Expr) -> Result<VariableValue, String> {
        let left_value = self.evaluate_expression(left)?;
        let right_value = self.evaluate_expression(right)?;

        match operator {
            BinaryOperator::Addition => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Int(a.checked_add(b).ok_or("Addition returns number too large")?)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Float(a + b)),
                    _ => Err("Addition operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::Subtraction => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Int(a.checked_sub(b).ok_or("Subtraction returns number too large")?)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Float(a - b)),
                    _ => Err("Subtraction operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::Multiplication => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Int(a.checked_mul(b).ok_or("Multiplication returns number too large")?)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Float(a * b)),
                    _ => Err("Multiplication operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::Division => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Int(a.checked_div(b).ok_or("Division returns number too large")?)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Float(a / b)),
                    _ => Err("Division operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::Modulo => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Int(a.checked_rem(b).ok_or("Modulo returns number too large")?)),
                    _ => Err("Modulo operator can only be applied to int".to_string()),
                }
            },
            BinaryOperator::Power => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => {
                        if b < 0 {
                            return Err("Power exponent cannot be negative for integer base".to_string());
                        }
                        let b_u32 = u32::try_from(b).map_err(|_| "Power exponent too large".to_string())?;
                        let result = a.checked_pow(b_u32).ok_or("Power returns number too large")?;
                        Ok(VariableValue::Int(result))
                    },
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Float(a.powf(b))),
                    _ => Err("Power operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::Equal => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Bool(a == b)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Bool(a == b)),
                    (VariableValue::String(a), VariableValue::String(b)) => Ok(VariableValue::Bool(a == b)),
                    (VariableValue::Bool(a), VariableValue::Bool(b)) => Ok(VariableValue::Bool(a == b)),
                    _ => Ok(VariableValue::Bool(false)) // Different types are considered not equal
                }
            },
            BinaryOperator::NotEqual => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Bool(a != b)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Bool(a != b)),
                    (VariableValue::String(a), VariableValue::String(b)) => Ok(VariableValue::Bool(a != b)),
                    (VariableValue::Bool(a), VariableValue::Bool(b)) => Ok(VariableValue::Bool(a != b)),
                    _ => Ok(VariableValue::Bool(true)) // Different types are considered not equal
                }
            },
            BinaryOperator::Greater => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Bool(a > b)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Bool(a > b)),
                    _ => Err("Greater operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::Less => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Bool(a < b)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Bool(a < b)),
                    _ => Err("Less operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::GreaterEqual => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Bool(a >= b)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Bool(a >= b)),
                    _ => Err("Greater or equal operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::LessEqual => {
                match (left_value, right_value) {
                    (VariableValue::Int(a), VariableValue::Int(b)) => Ok(VariableValue::Bool(a <= b)),
                    (VariableValue::Float(a), VariableValue::Float(b)) => Ok(VariableValue::Bool(a <= b)),
                    _ => Err("Less or equal operator can only be applied to int or float".to_string()),
                }
            },
            BinaryOperator::And => {
                match (left_value, right_value) {
                    (VariableValue::Bool(a), VariableValue::Bool(b)) => Ok(VariableValue::Bool(a && b)),
                    _ => Err("And operator can only be applied to bool".to_string()),
                }
            },
            BinaryOperator::Or => {
                match (left_value, right_value) {
                    (VariableValue::Bool(a), VariableValue::Bool(b)) => Ok(VariableValue::Bool(a || b)),
                    _ => Err("Or operator can only be applied to bool".to_string()),
                }
            },
        }
    }

    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<VariableValue, String> {
        match expr {
            Expr::Atom(atom) => self.evaluate_atom(atom),
            Expr::UnaryOperation { operator, operand } => self.evaluate_unary_operation(operator, operand),
            Expr::BinaryOperation { left, operator, right } => self.evaluate_binary_operation(left, operator, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::parser::abstract_syntax_tree::common::Identifier;

    use super::*;

    // Helper function to create a test RuntimeEnvironment
    fn create_test_env() -> RuntimeEnvironment {
        RuntimeEnvironment::new()
    }

    // ============ Tests for evaluate_atom ============

    #[test]
    fn test_evaluate_atom_integer_literal() {
        let mut env = create_test_env();
        let atom = ExprAtom::Literal(Literal::Int(42));
        let result = env.evaluate_atom(&atom);
        assert_eq!(result, Ok(VariableValue::Int(42)));
    }

    #[test]
    fn test_evaluate_atom_float_literal() {
        let mut env = create_test_env();
        let atom = ExprAtom::Literal(Literal::Float(3.14));
        let result = env.evaluate_atom(&atom);
        assert_eq!(result, Ok(VariableValue::Float(3.14)));
    }

    #[test]
    fn test_evaluate_atom_string_literal() {
        let mut env = create_test_env();
        let atom = ExprAtom::Literal(Literal::String("hello".to_string()));
        let result = env.evaluate_atom(&atom);
        assert_eq!(result, Ok(VariableValue::String("hello".to_string())));
    }

    #[test]
    fn test_evaluate_atom_bool_literal_true() {
        let mut env = create_test_env();
        let atom = ExprAtom::Literal(Literal::Bool(true));
        let result = env.evaluate_atom(&atom);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_evaluate_atom_bool_literal_false() {
        let mut env = create_test_env();
        let atom = ExprAtom::Literal(Literal::Bool(false));
        let result = env.evaluate_atom(&atom);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    // ============ Tests for Unary Operations ============

    #[test]
    fn test_unary_negation_int() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Neg,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(-5)));
    }

    #[test]
    fn test_unary_negation_negative_int() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Neg,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(-10)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(10)));
    }

    #[test]
    fn test_unary_negation_float() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Neg,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(2.5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(-2.5)));
    }

    #[test]
    fn test_unary_negation_bool_fails() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Neg,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Negation operator can only be applied to int or float");
    }

    #[test]
    fn test_unary_not_true() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_unary_not_false() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_unary_not_int_fails() {
        let mut env = create_test_env();
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not operator can only be applied to bool");
    }

    // ============ Tests for Binary Operations - Arithmetic ============

    #[test]
    fn test_addition_integers() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(8)));
    }

    #[test]
    fn test_addition_floats() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(2.5)))),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(1.5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(4.0)));
    }

    #[test]
    fn test_addition_mixed_types_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(2.5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtraction_integers() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10)))),
            operator: BinaryOperator::Subtraction,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(7)));
    }

    #[test]
    fn test_subtraction_negative_result() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
            operator: BinaryOperator::Subtraction,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(-7)));
    }

    #[test]
    fn test_multiplication_integers() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))),
            operator: BinaryOperator::Multiplication,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(20)));
    }

    #[test]
    fn test_multiplication_by_zero() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(42)))),
            operator: BinaryOperator::Multiplication,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(0)));
    }

    #[test]
    fn test_division_integers() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(20)))),
            operator: BinaryOperator::Division,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(5)));
    }

    #[test]
    fn test_division_by_zero_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(20)))),
            operator: BinaryOperator::Division,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_integers() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(17)))),
            operator: BinaryOperator::Modulo,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(2)));
    }

    #[test]
    fn test_modulo_float_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(17.5)))),
            operator: BinaryOperator::Modulo,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(5.0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Modulo operator can only be applied to int");
    }

    // ============ Tests for Power Operation ============

    #[test]
    fn test_power_positive_integers() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(8)));
    }

    #[test]
    fn test_power_zero_exponent() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(1)));
    }

    #[test]
    fn test_power_one_exponent() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(42)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(42)));
    }

    #[test]
    fn test_power_negative_exponent_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(-3)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Power exponent cannot be negative for integer base");
    }

    #[test]
    fn test_power_large_exponent_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(i64::MAX)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Power exponent too large");
    }

    #[test]
    fn test_power_overflow_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1000)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1000)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Power returns number too large");
    }

    #[test]
    fn test_power_floats() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(2.0)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(3.0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(8.0)));
    }

    #[test]
    fn test_power_float_negative_exponent() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(2.0)))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(-2.0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(0.25)));
    }

    // ============ Tests for Comparison Operations ============

    #[test]
    fn test_equal_integers_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Equal,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_equal_integers_false() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Equal,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_equal_strings() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::String("hello".to_string())))),
            operator: BinaryOperator::Equal,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::String("hello".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_equal_different_types() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Equal,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Float(5.0)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_not_equal_integers_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::NotEqual,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_not_equal_integers_false() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::NotEqual,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_greater_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10)))),
            operator: BinaryOperator::Greater,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_greater_false() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::Greater,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_less_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
            operator: BinaryOperator::Less,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(7)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_less_false() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(10)))),
            operator: BinaryOperator::Less,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_greater_equal_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::GreaterEqual,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_less_equal_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            operator: BinaryOperator::LessEqual,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    // ============ Tests for Logical Operations ============

    #[test]
    fn test_and_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
            operator: BinaryOperator::And,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_and_false() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
            operator: BinaryOperator::And,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_and_int_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1)))),
            operator: BinaryOperator::And,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1)))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "And operator can only be applied to bool");
    }

    #[test]
    fn test_or_true() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_or_false() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_or_string_fails() {
        let mut env = create_test_env();
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::String("a".to_string())))),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::String("b".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
    }

    // ============ Tests for Complex Nested Expressions ============

    #[test]
    fn test_nested_arithmetic() {
        let mut env = create_test_env();
        // (5 + 3) * 2
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
                operator: BinaryOperator::Addition,
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
            }),
            operator: BinaryOperator::Multiplication,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(16)));
    }

    #[test]
    fn test_nested_logical() {
        let mut env = create_test_env();
        // (true && false) || true
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
                operator: BinaryOperator::And,
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(false)))),
            }),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Bool(true)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_nested_comparison_and_logic() {
        let mut env = create_test_env();
        // (5 > 3) && (2 < 4)
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
                operator: BinaryOperator::Greater,
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
            }),
            operator: BinaryOperator::And,
            right: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
                operator: BinaryOperator::Less,
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))),
            }),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_complex_mixed_operations() {
        let mut env = create_test_env();
        // 2^3 + 4 * 2 - 1
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::BinaryOperation {
                    left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
                    operator: BinaryOperator::Power,
                    right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(3)))),
                }),
                operator: BinaryOperator::Addition,
                right: Box::new(Expr::BinaryOperation {
                    left: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(4)))),
                    operator: BinaryOperator::Multiplication,
                    right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
                }),
            }),
            operator: BinaryOperator::Subtraction,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(1)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(15))); // 8 + 8 - 1 = 15
    }

    // ============ Tests for Variables in Expressions ============

    #[test]
    fn test_variable_integer() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(42)).unwrap();
        
        let expr = Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())));
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(42)));
    }

    #[test]
    fn test_variable_float() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("pi".to_string(), VariableValue::Float(3.14159)).unwrap();
        
        let expr = Expr::Atom(ExprAtom::Identifier(Identifier("pi".to_string())));
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(3.14159)));
    }

    #[test]
    fn test_variable_string() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("name".to_string(), VariableValue::String("Alice".to_string())).unwrap();
        
        let expr = Expr::Atom(ExprAtom::Identifier(Identifier("name".to_string())));
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::String("Alice".to_string())));
    }

    #[test]
    fn test_variable_bool() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("flag".to_string(), VariableValue::Bool(true)).unwrap();
        
        let expr = Expr::Atom(ExprAtom::Identifier(Identifier("flag".to_string())));
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_undefined_variable() {
        let mut env = create_test_env();
        let expr = Expr::Atom(ExprAtom::Identifier(Identifier("undefined".to_string())));
        let result = env.evaluate_expression(&expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_variable_in_arithmetic() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(5)).unwrap();
        env.memory_manager.declare_variable("y".to_string(), VariableValue::Int(3)).unwrap();
        
        // x + y
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())))),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("y".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(8)));
    }

    #[test]
    fn test_variable_subtraction() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("a".to_string(), VariableValue::Int(10)).unwrap();
        env.memory_manager.declare_variable("b".to_string(), VariableValue::Int(4)).unwrap();
        
        // a - b
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("a".to_string())))),
            operator: BinaryOperator::Subtraction,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("b".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(6)));
    }

    #[test]
    fn test_variable_multiplication() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("base".to_string(), VariableValue::Int(7)).unwrap();
        env.memory_manager.declare_variable("multiplier".to_string(), VariableValue::Int(6)).unwrap();
        
        // base * multiplier
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("base".to_string())))),
            operator: BinaryOperator::Multiplication,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("multiplier".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(42)));
    }

    #[test]
    fn test_variable_division() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("dividend".to_string(), VariableValue::Int(20)).unwrap();
        env.memory_manager.declare_variable("divisor".to_string(), VariableValue::Int(4)).unwrap();
        
        // dividend / divisor
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("dividend".to_string())))),
            operator: BinaryOperator::Division,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("divisor".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(5)));
    }

    #[test]
    fn test_variable_power() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("base".to_string(), VariableValue::Int(2)).unwrap();
        env.memory_manager.declare_variable("exponent".to_string(), VariableValue::Int(4)).unwrap();
        
        // base ^ exponent
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("base".to_string())))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("exponent".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(16)));
    }

    #[test]
    fn test_variable_modulo() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("dividend".to_string(), VariableValue::Int(17)).unwrap();
        env.memory_manager.declare_variable("divisor".to_string(), VariableValue::Int(5)).unwrap();
        
        // dividend % divisor
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("dividend".to_string())))),
            operator: BinaryOperator::Modulo,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("divisor".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(2)));
    }

    #[test]
    fn test_variable_in_comparison() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("left".to_string(), VariableValue::Int(10)).unwrap();
        env.memory_manager.declare_variable("right".to_string(), VariableValue::Int(5)).unwrap();
        
        // left > right
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("left".to_string())))),
            operator: BinaryOperator::Greater,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("right".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_variable_equality() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(7)).unwrap();
        env.memory_manager.declare_variable("y".to_string(), VariableValue::Int(7)).unwrap();
        
        // x == y
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())))),
            operator: BinaryOperator::Equal,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("y".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_variable_in_boolean_logic() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("flag1".to_string(), VariableValue::Bool(true)).unwrap();
        env.memory_manager.declare_variable("flag2".to_string(), VariableValue::Bool(false)).unwrap();
        
        // flag1 && flag2
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("flag1".to_string())))),
            operator: BinaryOperator::And,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("flag2".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_variable_in_logical_or() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("a".to_string(), VariableValue::Bool(false)).unwrap();
        env.memory_manager.declare_variable("b".to_string(), VariableValue::Bool(true)).unwrap();
        
        // a || b
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("a".to_string())))),
            operator: BinaryOperator::Or,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("b".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_variable_with_unary_negation() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(5)).unwrap();
        
        // -x
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Neg,
            operand: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(-5)));
    }

    #[test]
    fn test_variable_with_unary_not() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("flag".to_string(), VariableValue::Bool(true)).unwrap();
        
        // !flag
        let expr = Expr::UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("flag".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(false)));
    }

    #[test]
    fn test_complex_expression_with_variables() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(3)).unwrap();
        env.memory_manager.declare_variable("y".to_string(), VariableValue::Int(2)).unwrap();
        env.memory_manager.declare_variable("z".to_string(), VariableValue::Int(4)).unwrap();
        
        // (x + y) * z
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())))),
                operator: BinaryOperator::Addition,
                right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("y".to_string())))),
            }),
            operator: BinaryOperator::Multiplication,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("z".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(20))); // (3 + 2) * 4 = 20
    }

    #[test]
    fn test_variable_with_literal_mixed() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(10)).unwrap();
        
        // x * 5 + 2
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("x".to_string())))),
                operator: BinaryOperator::Multiplication,
                right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(5)))),
            }),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Literal(Literal::Int(2)))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(52))); // 10 * 5 + 2 = 52
    }

    #[test]
    fn test_float_variable_arithmetic() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("a".to_string(), VariableValue::Float(2.5)).unwrap();
        env.memory_manager.declare_variable("b".to_string(), VariableValue::Float(1.5)).unwrap();
        
        // a + b
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("a".to_string())))),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("b".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(4.0)));
    }

    #[test]
    fn test_float_variable_power() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("base".to_string(), VariableValue::Float(2.0)).unwrap();
        env.memory_manager.declare_variable("exp".to_string(), VariableValue::Float(-2.0)).unwrap();
        
        // base ^ exp
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("base".to_string())))),
            operator: BinaryOperator::Power,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("exp".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Float(0.25))); // 2^-2 = 0.25
    }

    #[test]
    fn test_string_variable_equality() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("s1".to_string(), VariableValue::String("hello".to_string())).unwrap();
        env.memory_manager.declare_variable("s2".to_string(), VariableValue::String("hello".to_string())).unwrap();
        
        // s1 == s2
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("s1".to_string())))),
            operator: BinaryOperator::Equal,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("s2".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_string_variable_inequality() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("s1".to_string(), VariableValue::String("hello".to_string())).unwrap();
        env.memory_manager.declare_variable("s2".to_string(), VariableValue::String("world".to_string())).unwrap();
        
        // s1 != s2
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("s1".to_string())))),
            operator: BinaryOperator::NotEqual,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("s2".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Bool(true)));
    }

    #[test]
    fn test_multiple_variables_in_complex_expr() {
        let mut env = create_test_env();
        env.memory_manager.declare_variable("a".to_string(), VariableValue::Int(2)).unwrap();
        env.memory_manager.declare_variable("b".to_string(), VariableValue::Int(3)).unwrap();
        env.memory_manager.declare_variable("c".to_string(), VariableValue::Int(4)).unwrap();
        
        // (a ^ b) + c
        let expr = Expr::BinaryOperation {
            left: Box::new(Expr::BinaryOperation {
                left: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("a".to_string())))),
                operator: BinaryOperator::Power,
                right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("b".to_string())))),
            }),
            operator: BinaryOperator::Addition,
            right: Box::new(Expr::Atom(ExprAtom::Identifier(Identifier("c".to_string())))),
        };
        let result = env.evaluate_expression(&expr);
        assert_eq!(result, Ok(VariableValue::Int(12))); // 2^3 + 4 = 8 + 4 = 12
    }
}