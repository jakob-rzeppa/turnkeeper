use crate::application::game::plugin::{parser::abstract_syntax_tree::statement::Assignment, runtime::RuntimeEnvironment};


impl RuntimeEnvironment {
    pub fn execute_assignment(&mut self, assignment: &Assignment) -> Result<(), String> {
        let name = assignment.target.to_string();
        let value = self.evaluate_expression(&assignment.value)?;

        self.memory_manager.assign_variable(name, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::{parser::abstract_syntax_tree::{common::Identifier, expression::{Expr, ExprAtom, Literal}}, runtime::memory::VariableValue};

    use super::*;

    #[test]
    fn test_assignment() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(42)).unwrap();

        let assignment = Assignment {
            target: Identifier("x".to_string()),
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(100))),
        };
        assert!(env.execute_assignment(&assignment).is_ok());
        let stored_value = env.memory_manager.get_variable("x").unwrap();
        assert_eq!(stored_value, &VariableValue::Int(100));
    }

    #[test]
    fn test_assignment_missing_variable() {
        let mut env = RuntimeEnvironment::new();

        let assignment = Assignment {
            target: Identifier("y".to_string()),
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(200))),
        };
        assert!(env.execute_assignment(&assignment).is_err());
    }

    #[test]
    fn test_assignment_type_mismatch() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager.declare_variable("z".to_string(), VariableValue::Int(42)).unwrap();

        let assignment = Assignment {
            target: Identifier("z".to_string()),
            value: Expr::Atom(ExprAtom::Literal(Literal::String("not an integer".to_string()))),
        };
        assert!(env.execute_assignment(&assignment).is_err());
    }
}