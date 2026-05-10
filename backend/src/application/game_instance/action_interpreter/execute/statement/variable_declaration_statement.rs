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
            abstract_syntax_tree::statement::VariableDeclarationStatement,
            value_objects::data::{ Datatype, Value },
        },
    },
};

impl Executable<()> for VariableDeclarationStatement {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<(), RuntimeError> {
        let var_type = self.datatype();
        let expression = self.value();
        let value = self.value().execute(env).await?;

        // Type checking
        match (&var_type, &value) {
            | (Datatype::Int, Value::Int(_))
            | (Datatype::Float, Value::Float(_))
            | (Datatype::String, Value::String(_))
            | (Datatype::Bool, Value::Bool(_)) => {
                if env.declare_variable(self.name().to_string(), value) == true {
                    Ok(())
                } else {
                    Err(RuntimeError::VariableAlreadyDeclared {
                        name: self.name().to_string(),
                        pos: expression.position(),
                    })
                }
            }
            _ =>
                Err(RuntimeError::TypeMismatch {
                    expected: format!("type {} for variable '{}'", var_type, self.name()),
                    found: value,
                    pos: expression.position(),
                }),
        }
    }
}
