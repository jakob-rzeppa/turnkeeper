use backend_derive::execute_debug;

use crate::{
    application::action_interpreter::{
        error::RuntimeError,
        execute::Executable,
        runtime_env::RuntimeEnvironment,
    },
    domain::game::{ abstract_syntax_tree::expression::Expression, value_objects::data::Value },
};

pub mod atom;
pub mod binary;
pub mod unary;

impl Executable<Value> for Expression {
    #[execute_debug]
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<Value, RuntimeError> {
        // We need to box the future returned by the inner execute to add indirection to the async call, since we use recursion in the expression evaluation (e.g. for binary and unary expressions).
        match self {
            Expression::Atom(atom) => Box::pin(atom.execute(env)).await,
            Expression::Unary(unary_expr) => Box::pin(unary_expr.execute(env)).await,
            Expression::Binary(binary_expr) => Box::pin(binary_expr.execute(env)).await,
        }
    }
}
