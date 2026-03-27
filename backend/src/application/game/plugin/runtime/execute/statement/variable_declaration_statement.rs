use crate::application::game::plugin::{parser::old_abstract_syntax_tree::{datatype::{Datatype, DatatypeVariant}, statement::VariableDeclaration}, runtime::{RuntimeEnvironment, memory::VariableValue}};

impl RuntimeEnvironment {
    pub fn execute_variable_declaration_statement(&mut self, element: &VariableDeclaration) -> Result<(), String> {
        let name = element.name.name.clone();
        let var_type = &element.datatype;
        let value = self.evaluate_expression(&element.value)?;

        // Type checking
        match (&var_type.variant, &value) {
            (DatatypeVariant::Int, VariableValue::Int(_)) |
            (DatatypeVariant::Float, VariableValue::Float(_)) |
            (DatatypeVariant::String, VariableValue::String(_)) |
            (DatatypeVariant::Bool, VariableValue::Bool(_)) => {
                self.memory_manager.declare_variable(name, value)
            },
            _ => Err(format!("Type mismatch: cannot assign value of type {:?} to variable of type {:?}", value, var_type)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::{common::Position, parser::old_abstract_syntax_tree::{datatype::{Datatype, DatatypeVariant}, expression::{Expr, ExprAtom, Literal}, identifier::Identifier}};
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut env = RuntimeEnvironment::new();
        let var_decl = VariableDeclaration {
            name: Identifier::new("x".to_string(), Position::new(0, 0)),
            datatype: Datatype::new(DatatypeVariant::Int, Position::new(0, 0)),
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(42))),
        };

        let result = env.execute_variable_declaration_statement(&var_decl);
        assert!(result.is_ok());
        let stored_value = env.memory_manager.get_variable("x").unwrap();
        assert_eq!(stored_value, &VariableValue::Int(42));
    }

    #[test]
    fn test_variable_declaration_type_mismatch() {
        let mut env = RuntimeEnvironment::new();
        let var_decl = VariableDeclaration {
            name: Identifier::new("x".to_string(), Position::new(0, 0)),
            datatype: Datatype::new(DatatypeVariant::Int, Position::new(0, 0)),
            value: Expr::Atom(ExprAtom::Literal(Literal::String("not an integer".to_string()))),
        };

        let result = env.execute_variable_declaration_statement(&var_decl);
        assert!(result.is_err());
        let error_message = result.err().unwrap();
        assert!(error_message.contains("Type mismatch"));
    }

    #[test]
    fn test_variable_declaration_shadowing() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(42)).unwrap();
        env.memory_manager.push_scope(); // Enter a new scope

        let var_decl = VariableDeclaration {
            name: Identifier::new("x".to_string(), Position::new(0, 0)),
            datatype: Datatype::new(DatatypeVariant::Int, Position::new(0, 0)),
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(100))),
        };

        let result = env.execute_variable_declaration_statement(&var_decl);
        assert!(result.is_ok());
        let stored_value = env.memory_manager.get_variable("x").unwrap();
        assert_eq!(stored_value, &VariableValue::Int(100)); // The inner scope should shadow the outer variable

        env.memory_manager.pop_scope(); // Exit the inner scope
        let stored_value = env.memory_manager.get_variable("x").unwrap();
        assert_eq!(stored_value, &VariableValue::Int(42)); // The outer variable should still be accessible after popping the inner scope
    }

    #[test]
    fn test_variable_declaration_same_scope_fails() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager.declare_variable("x".to_string(), VariableValue::Int(42)).unwrap();

        let var_decl = VariableDeclaration {
            name: Identifier::new("x".to_string(), Position::new(0, 0)),
            datatype: Datatype::new(DatatypeVariant::Int, Position::new(0, 0)),
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(100))),
        };

        let result = env.execute_variable_declaration_statement(&var_decl);
        assert!(result.is_err());

        let old_value = env.memory_manager.get_variable("x").unwrap();
        assert_eq!(old_value, &VariableValue::Int(42)); // The original variable should still be unchanged after the failed declaration attempt
    }
}