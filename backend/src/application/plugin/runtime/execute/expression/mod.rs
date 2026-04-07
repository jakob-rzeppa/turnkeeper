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
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<VariableValue, RuntimeError> {
        match self {
            Expression::Atom(atom) => atom.execute(env),
            Expression::Unary(unary_expr) => unary_expr.execute(env),
            Expression::Binary(binary_expr) => binary_expr.execute(env),
        }
    }
}
