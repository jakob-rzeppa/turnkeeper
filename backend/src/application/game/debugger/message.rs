use crate::application::{ action_interpreter::runtime_env::RuntimeEnvironmentProjection };

#[derive(Debug, Clone, serde::Serialize)]
pub enum DebuggerMessage {
    Finished {
        environment: RuntimeEnvironmentProjection,
    },
    BreakpointHit {
        environment: RuntimeEnvironmentProjection,
    },
    SetupError {
        message: String,
    },
    ParsingError {
        message: String,
    },
    RuntimeError {
        message: String,
    },
    InternalError {
        message: String,
    },
}
