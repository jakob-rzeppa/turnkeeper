use crate::application::plugin::{
    parser::abstract_syntax_tree::expression::Expression,
    runtime::{
        RuntimeEnvironment, error::RuntimeError, execute::Executable, memory::values::VariableValue,
    },
};

pub mod atom;
pub mod binary;
pub mod unary;

impl Executable<VariableValue> for Expression {
    async fn execute(&self, env: &mut RuntimeEnvironment) -> Result<VariableValue, RuntimeError> {
        // We need to box the future returned by the inner execute to add indirection to the async call, since we use recursion in the expression evaluation (e.g. for binary and unary expressions).
        match self {
            Expression::Atom(atom) => Box::pin(atom.execute(env)).await,
            Expression::Unary(unary_expr) => Box::pin(unary_expr.execute(env)).await,
            Expression::Binary(binary_expr) => Box::pin(binary_expr.execute(env)).await,
        }
    }
}
