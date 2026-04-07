use crate::application::plugin::{
    parser::abstract_syntax_tree::{
        Positioned, atom::datatype::Datatype,
        statement::variable_declaration::VariableDeclarationStatement,
    },
    runtime::{
        RuntimeEnvironment,
        error::RuntimeError,
        execute::Executable,
        memory::{identifier::Identifier, values::VariableValue},
    },
};

impl Executable<()> for VariableDeclarationStatement {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let identifier = Identifier::from(self.identifier());
        let var_type = self.datatype();
        let expression = self.value();
        let value = self.value().execute(env)?;

        // Type checking
        match (&var_type, &value) {
            (Datatype::Integer, VariableValue::Int(_))
            | (Datatype::Float, VariableValue::Float(_))
            | (Datatype::String, VariableValue::String(_))
            | (Datatype::Boolean, VariableValue::Bool(_)) => env
                .memory_manager
                .declare_variable(identifier, value)
                .map_err(|err| RuntimeError::from_memory_error(err, expression.position())),
            _ => Err(RuntimeError::TypeMismatch {
                expected: format!("type {} for variable '{}'", var_type, identifier),
                found: value,
                pos: expression.position(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::plugin::parser::abstract_syntax_tree::expression::Expression;

    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut env = RuntimeEnvironment::new();
        let var_decl = VariableDeclarationStatement::new(
            "x",
            Datatype::Integer,
            Expression::new_atom_literal_int(42, 0, 0),
            0,
            0,
        );

        let result = var_decl.execute(&mut env);
        assert!(result.is_ok());
        let stored_value = env
            .memory_manager
            .get_variable(&Identifier::new("x".to_string()))
            .unwrap();
        assert_eq!(stored_value, &VariableValue::Int(42));
    }

    #[test]
    fn test_variable_declaration_type_mismatch() {
        let mut env = RuntimeEnvironment::new();
        let var_decl = VariableDeclarationStatement::new(
            "x",
            Datatype::Integer,
            Expression::new_atom_literal_string("not an integer".to_string(), 5, 0),
            0,
            0,
        );

        let result = var_decl.execute(&mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_variable_declaration_shadowing() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager
            .declare_variable(Identifier::new("x".to_string()), VariableValue::Int(42))
            .unwrap();
        env.memory_manager.push_scope(); // Enter a new scope

        let var_decl = VariableDeclarationStatement::new(
            "x",
            Datatype::Integer,
            Expression::new_atom_literal_int(100, 0, 0),
            0,
            0,
        );

        let result = var_decl.execute(&mut env);
        assert!(result.is_ok());
        let stored_value = env
            .memory_manager
            .get_variable(&Identifier::new("x".to_string()))
            .unwrap();
        assert_eq!(stored_value, &VariableValue::Int(100)); // The inner scope should shadow the outer variable

        env.memory_manager.pop_scope(); // Exit the inner scope
        let stored_value = env
            .memory_manager
            .get_variable(&Identifier::new("x".to_string()))
            .unwrap();
        assert_eq!(stored_value, &VariableValue::Int(42)); // The outer variable should still be accessible after popping the inner scope
    }

    #[test]
    fn test_variable_declaration_same_scope_fails() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager
            .declare_variable(Identifier::new("x".to_string()), VariableValue::Int(42))
            .unwrap();

        let var_decl = VariableDeclarationStatement::new(
            "x",
            Datatype::Integer,
            Expression::new_atom_literal_int(100, 0, 0),
            0,
            0,
        );

        let result = var_decl.execute(&mut env);
        assert!(result.is_err());

        let old_value = env
            .memory_manager
            .get_variable(&Identifier::new("x".to_string()))
            .unwrap();
        assert_eq!(old_value, &VariableValue::Int(42)); // The original variable should still be unchanged after the failed declaration attempt
    }
}
