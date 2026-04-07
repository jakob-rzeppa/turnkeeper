use crate::application::plugin::runtime::{RuntimeEnvironment, error::RuntimeError};

mod expression;
// mod function_evaluation;
mod statement;

pub trait Executable<R> {
    fn execute(&self, env: &mut RuntimeEnvironment) -> Result<R, RuntimeError>;
}
