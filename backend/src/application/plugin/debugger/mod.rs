use tokio::sync::mpsc;

use crate::application::plugin::{debugger::commands::DebugCommand, runtime::RuntimeEnvironment};

pub mod commands;

pub fn debug(code: &str) {
    println!("Debugging code:\n{}", code);

    let mut runtime_env = RuntimeEnvironment::new();

    let (s, r) = mpsc::unbounded_channel::<DebugCommand>();
}
