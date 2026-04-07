use crate::application::plugin::{
    parser::abstract_syntax_tree::{Positioned, statement::assignment::AssignmentStatement},
    runtime::{
        RuntimeEnvironment, error::RuntimeError, execute::Executable,
        memory::identifier::Identifier,
    },
};

impl Executable<()> for AssignmentStatement {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let name = Identifier::from(self.identifier());
        let value = self.value().execute(env)?;

        env.memory_manager
            .assign_variable(name, value)
            .map_err(|err| RuntimeError::from_memory_error(err, self.position()))
    }
}

#[cfg(test)]
mod tests {

    use crate::application::plugin::{
        parser::abstract_syntax_tree::expression::Expression,
        runtime::memory::{identifier::Identifier, values::VariableValue},
    };

    use super::*;

    #[test]
    fn test_assignment() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager
            .declare_variable(Identifier::new("x".to_string()), VariableValue::Int(42))
            .unwrap();

        let assignment =
            AssignmentStatement::new("x", Expression::new_atom_literal_int(100, 0, 0), 0, 0);
        assert!(assignment.execute(&mut env).is_ok());
        let stored_value = env
            .memory_manager
            .get_variable(&Identifier::new("x".to_string()))
            .unwrap();
        assert_eq!(stored_value, &VariableValue::Int(100));
    }

    #[test]
    fn test_assignment_missing_variable() {
        let mut env = RuntimeEnvironment::new();

        let assignment =
            AssignmentStatement::new("y", Expression::new_atom_literal_int(200, 0, 0), 0, 0);
        assert!(assignment.execute(&mut env).is_err());
    }

    #[test]
    fn test_assignment_type_mismatch() {
        let mut env = RuntimeEnvironment::new();
        env.memory_manager
            .declare_variable(Identifier::new("z".to_string()), VariableValue::Int(42))
            .unwrap();

        let assignment = AssignmentStatement::new(
            "z",
            Expression::new_atom_literal_string("not an integer".to_string(), 0, 0),
            0,
            0,
        );
        assert!(assignment.execute(&mut env).is_err());
    }
}
