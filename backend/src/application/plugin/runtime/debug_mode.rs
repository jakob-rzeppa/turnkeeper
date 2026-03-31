use tokio::sync::{mpsc, watch};

use crate::application::plugin::{
    debugger::commands::DebugCommand, runtime::debug_state::DebugState,
};

pub struct DebugMode {
    /// List of breakpoints, represented as line numbers in the source code.
    breakpoints: Vec<usize>,
    /// Indicates whether the execution is currently halted at a breakpoint.
    /// If so, the runtime should only step when the user explicitly commands it.
    halted: bool,

    /// Channel for receiving debug commands from the user interface (e.g., step, continue).
    command_receiver: mpsc::UnboundedReceiver<DebugCommand>,
    /// Channel for sending the current debug state back to the user interface.
    state_sender: watch::Sender<DebugState>,
}

pub enum StepResult {
    SteppedInto,
    SteppedOver,
    Continued,
}

impl DebugMode {
    pub fn new(
        breakpoints: Vec<usize>,
        command_receiver: mpsc::UnboundedReceiver<DebugCommand>,
        state_sender: watch::Sender<DebugState>,
    ) -> Self {
        Self {
            breakpoints,
            halted: false,
            command_receiver,
            state_sender,
        }
    }

    pub fn should_halt(&self, line: usize) -> bool {
        self.breakpoints.contains(&line)
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn resume(&mut self) {
        self.halted = false;
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// Waits for a debug command from the user interface.
    ///
    ///
    pub async fn wait_for_step(&mut self) -> StepResult {
        self.command_receiver.recv().await;

        StepResult::SteppedInto
    }
}
