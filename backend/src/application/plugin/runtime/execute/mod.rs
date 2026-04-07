use crate::application::plugin::runtime::{
    RuntimeEnvironment, debug::DebugEnvironment, error::RuntimeError,
};

mod expression;
// mod function_evaluation;
mod statement;

pub trait Executable<R> {
    fn execute(
        &self,
        env: &mut RuntimeEnvironment,
    ) -> impl std::future::Future<Output = Result<R, RuntimeError>>;

    fn execute_debug(
        &self,
        env: &mut RuntimeEnvironment,
        debug_env: &mut DebugEnvironment,
    ) -> impl std::future::Future<Output = Result<R, RuntimeError>>;
}
