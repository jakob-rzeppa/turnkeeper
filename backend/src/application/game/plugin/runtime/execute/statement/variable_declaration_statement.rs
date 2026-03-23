use crate::application::game::plugin::{parser::abstract_syntax_tree::{common::Type, statement::VariableDeclaration}, runtime::{RuntimeEnvironment, memory::VariableValue}};

impl RuntimeEnvironment {
    pub fn execute_variable_declaration_statement(&mut self, element: &VariableDeclaration) -> Result<(), String> {
        let name = element.name.0.clone();
        let var_type = &element.datatype;
        let value = self.evaluate_expression(&element.value)?;

        // Type checking
        match (var_type, &value) {
            (Type::Int, VariableValue::Int(_)) |
            (Type::Float, VariableValue::Float(_)) |
            (Type::String, VariableValue::String(_)) |
            (Type::Bool, VariableValue::Bool(_)) => {
                self.memory_manager.declare_variable(name, value)
            },
            _ => Err(format!("Type mismatch: cannot assign value of type {:?} to variable of type {:?}", value, var_type)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::game::plugin::parser::abstract_syntax_tree::{common::Identifier, expression::{Expr, ExprAtom, Literal}};
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut env = RuntimeEnvironment::new();
        let var_decl = VariableDeclaration {
            name: Identifier("x".to_string()),
            datatype: Type::Int,
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
            name: Identifier("x".to_string()),
            datatype: Type::Int,
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
            name: Identifier("x".to_string()),
            datatype: Type::Int,
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
            name: Identifier("x".to_string()),
            datatype: Type::Int,
            value: Expr::Atom(ExprAtom::Literal(Literal::Int(100))),
        };

        let result = env.execute_variable_declaration_statement(&var_decl);
        assert!(result.is_err());

        let old_value = env.memory_manager.get_variable("x").unwrap();
        assert_eq!(old_value, &VariableValue::Int(42)); // The original variable should still be unchanged after the failed declaration attempt
    }
}