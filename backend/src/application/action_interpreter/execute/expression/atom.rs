use backend_derive::execute_debug;

use crate::{
    application::action_interpreter::{
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
            ExpressionAtom::FunctionCall { name, args, pos: _ } => {
                // Evaluate arguments
                let mut evaluated_args = Vec::new();
                for arg in args {
                    let evaluated_arg = arg.execute(env).await?;
                    evaluated_args.push(evaluated_arg);
                }

                // Placeholder
                println!("Executing function call: {} with args: {:?}", name, evaluated_args);
                Ok(Value::Null) // Placeholder return value
            }
        }
    }
}
