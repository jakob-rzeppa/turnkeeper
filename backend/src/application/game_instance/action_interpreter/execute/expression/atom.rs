use backend_derive::execute_debug;

use crate::{
    application::game_instance::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::{
        abstract_syntax_tree::expression::atom::ExpressionAtom,
        value_objects::data::Value,
    },
};

impl Executable<Value> for ExpressionAtom {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<Value, RuntimeError> {
        match self {
            ExpressionAtom::Literal(literal, _) => Ok(Value::from(literal.clone())),
            ExpressionAtom::Variable(var, pos) =>
                env.get_variable(var).ok_or_else(|| RuntimeError::UndefinedVariable {
                    name: var.clone(),
                    pos: *pos,
                }),
        }
    }
}
