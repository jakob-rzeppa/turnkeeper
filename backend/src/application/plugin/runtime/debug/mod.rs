use tokio::sync::oneshot;

use crate::application::{
    common::channels::mpsc::{MpscChannel, MpscChannelReceiver, MpscChannelSender},
    plugin::runtime::debug::{commands::DebugCommand, state::DebugState},
};

pub mod commands;
pub mod state;

pub struct DebugEnvironment {
    /// Line numbers where breakpoints are set
    breakpoints: Vec<usize>,

    breakpoint_channel_sender: MpscChannelSender<(DebugState, oneshot::Sender<DebugCommand>)>,

    is_halted: bool,
    is_stepping_over: bool,
    user_is_connected: bool,
}

impl DebugEnvironment {
    pub fn new(
        breakpoints: Vec<usize>,
    ) -> (
        Self,
        MpscChannelReceiver<(DebugState, oneshot::Sender<DebugCommand>)>,
    ) {
        let (breakpoint_channel_sender, breakpoint_channel_receiver) = MpscChannel::new();

        (
            Self {
                breakpoints,
                breakpoint_channel_sender,
                is_halted: false,
                is_stepping_over: false,
                user_is_connected: true,
            },
            breakpoint_channel_receiver,
        )
    }

    /// Waits at a breakpoint if the current line has a breakpoint set.
    ///
    /// Returns `true` if execution should call finish_step_over after the current execution unit to reset stepping over state, `false` otherwise.
    pub async fn wait(&mut self, current_line: usize, debug_state: DebugState) -> bool {
        // If the user has disconnected, we don't want to halt execution at breakpoints anymore -> just continue running until the end of the program
        if !self.user_is_connected {
            return false;
        }

        // If stepping over, we want to allow execution to continue without halting
        if self.is_stepping_over {
            return false;
        }

        if self.breakpoints.contains(&current_line) {
            self.is_halted = true;
        }

        if self.is_halted {
            let (command_sender, command_receiver) = oneshot::channel::<DebugCommand>();

            match self
                .breakpoint_channel_sender
                .send((debug_state, command_sender))
            {
                Ok(()) => {}
                Err(_) => {
                    eprintln!("User disconnected while debug execution was still running.");
                    self.user_is_connected = false;
                    return false;
                }
            }

            let command = command_receiver.await.unwrap_or_else(|e| {
                eprintln!("Failed to receive debug command: {}", e);
                DebugCommand::Continue // Default to continue on error
            });

            match command {
                DebugCommand::Continue => {
                    // Set is_halted to false to allow execution to continue until the next breakpoint (or the end of the program)
                    self.is_halted = false;
                    // We need to also step over the current execution unit to avoid handling inner units of the current line, since the user wants to continue execution until the next breakpoint (or the end of the program)
                    self.is_stepping_over = true;
                    return true;
                }
                DebugCommand::StepInto => {
                    // Step into will halt at the next execution unit -> so do nothing here
                }
                DebugCommand::StepOver => {
                    self.is_stepping_over = true;
                    // Indicate that we should step over the next execution unit -> call finish_step_over after the execution unit finishes to reset stepping over state
                    return true;
                }
            }
        }

        false
    }

    pub fn finish_step_over(&mut self) {
        self.is_stepping_over = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_debug_state() -> DebugState {
        DebugState { variables: vec![] }
    }

    #[tokio::test]
    async fn waits_at_breakpoint_and_continues_execution() {
        let (mut debug_env, mut breakpoint_rx) = DebugEnvironment::new(vec![10]);

        let wait_task = tokio::spawn(async move {
            let first_wait = debug_env.wait(10, empty_debug_state()).await;
            if first_wait {
                debug_env.finish_step_over();
            }
            let second_wait = debug_env.wait(11, empty_debug_state()).await;
            if second_wait {
                debug_env.finish_step_over();
            }
            (first_wait, second_wait)
        });

        let (state, command_sender) = breakpoint_rx
            .recv()
            .await
            .expect("Expected breakpoint message for line 10");
        assert_eq!(state.variables.len(), 0);
        command_sender.send(DebugCommand::Continue).unwrap();

        let (first_wait, second_wait) = wait_task.await.unwrap();
        assert!(first_wait);
        assert!(!second_wait);
    }

    #[tokio::test]
    async fn step_into_keeps_execution_halted_for_next_unit() {
        let (mut debug_env, mut breakpoint_rx) = DebugEnvironment::new(vec![10]);

        let wait_task = tokio::spawn(async move {
            let first_wait = debug_env.wait(10, empty_debug_state()).await;
            if first_wait {
                debug_env.finish_step_over();
            }
            let second_wait = debug_env.wait(11, empty_debug_state()).await;
            if second_wait {
                debug_env.finish_step_over();
            }
            (first_wait, second_wait)
        });

        let (_, first_command_sender) = breakpoint_rx
            .recv()
            .await
            .expect("Expected first breakpoint message");
        first_command_sender.send(DebugCommand::StepInto).unwrap();

        let (_, second_command_sender) = breakpoint_rx
            .recv()
            .await
            .expect("Expected second breakpoint message after step into");
        second_command_sender.send(DebugCommand::Continue).unwrap();

        let (first_wait, second_wait) = wait_task.await.unwrap();
        assert!(!first_wait);
        assert!(second_wait);
    }

    #[tokio::test]
    async fn step_over_skips_one_unit_then_halts_again() {
        let (mut debug_env, mut breakpoint_rx) = DebugEnvironment::new(vec![10, 12]);

        let wait_task = tokio::spawn(async move {
            let first_wait = debug_env.wait(10, empty_debug_state()).await;
            let second_wait = debug_env.wait(11, empty_debug_state()).await;
            if second_wait {
                debug_env.finish_step_over();
            }
            if first_wait {
                debug_env.finish_step_over();
            }
            let third_wait = debug_env.wait(12, empty_debug_state()).await;
            if third_wait {
                debug_env.finish_step_over();
            }
            (first_wait, second_wait, third_wait)
        });

        let (_, first_command_sender) = breakpoint_rx
            .recv()
            .await
            .expect("Expected first breakpoint message");
        first_command_sender.send(DebugCommand::StepOver).unwrap();

        let (_, second_command_sender) = breakpoint_rx
            .recv()
            .await
            .expect("Expected second breakpoint message after finishing step over");
        second_command_sender.send(DebugCommand::Continue).unwrap();

        let (first_wait, second_wait, third_wait) = wait_task.await.unwrap();
        assert!(first_wait);
        assert!(!second_wait);
        assert!(third_wait);
    }
}
