use crate::application::plugin::{
    parser::abstract_syntax_tree::{Positioned, expression::atom::ExpressionAtom},
    runtime::{
        RuntimeEnvironment,
        error::RuntimeError,
        execute::Executable,
        memory::{identifier::Identifier, values::VariableValue},
    },
};

impl Executable<VariableValue> for ExpressionAtom {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<VariableValue, RuntimeError> {
        match self {
            ExpressionAtom::Literal(literal) => Ok(VariableValue::from(literal.value())),
            ExpressionAtom::Variable(var) => env
                .memory_manager
                .get_variable(&&Identifier::from(var.identifier()))
                .map_err(|_| RuntimeError::VariableNotFound {
                    identifier: Identifier::from(var.identifier()),
                    pos: var.position(),
                })
                .cloned(),
            ExpressionAtom::FunctionCall(_) => unimplemented!(),
        }
    }
}
