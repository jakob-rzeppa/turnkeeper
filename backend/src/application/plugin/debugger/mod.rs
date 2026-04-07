use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use crate::application::{
    common::channels::mpsc::{MpscChannel, MpscChannelReceiver, MpscChannelSender},
    plugin::{
        parser::parse_source_code,
        runtime::{
            RuntimeEnvironment,
            debug::{DebugEnvironment, commands::DebugCommand, state::DebugState},
        },
    },
};

#[derive(Debug, Deserialize)]
pub enum IncomingDebuggerMessage {
    Initialize {
        code: String,
        breakpoints: Vec<usize>,
    },
    StepInto,
    StepOver,
    Continue,
}

#[derive(Debug, Serialize)]
pub enum OutgoingDebuggerMessage {
    BreakpointHit(DebugState),
    Log(String),
    Error(String),
    Finished,
}

pub struct PluginDebugger;

impl PluginDebugger {
    pub fn new() -> Self {
        Self
    }

    pub async fn debug(
        &self,
    ) -> (
        MpscChannelSender<IncomingDebuggerMessage>,
        MpscChannelReceiver<OutgoingDebuggerMessage>,
    ) {
        let (incoming_sender, mut incoming_receiver) = MpscChannel::new();
        let (outgoing_sender, outgoing_receiver) = MpscChannel::new();

        // Spawn the "middleman" task that will manage the debug session, forwarding messages between the runtime and the client
        tokio::spawn(async move {
            // First, wait for the Initialize message to get the code and breakpoints from the client
            let (code, breakpoints) = match incoming_receiver.recv().await {
                Some(IncomingDebuggerMessage::Initialize { code, breakpoints }) => {
                    (code, breakpoints)
                }
                _ => {
                    let _ = outgoing_sender.send(OutgoingDebuggerMessage::Error(
                        "Expected Initialize message".to_string(),
                    ));
                    return;
                }
            };

            // Create the runtime environment and parse the source code into an AST.
            let mut runtime_env = RuntimeEnvironment::new();
            let ast = match parse_source_code(&code) {
                Ok(ast) => ast,
                Err(err) => {
                    let _ = outgoing_sender.send(OutgoingDebuggerMessage::Error(format!(
                        "Failed to parse code: {err}",
                    )));
                    return;
                }
            };

            // Create the debug environment, which will manage breakpoints and communication with the runtime.
            let (mut debug_env, mut breakpoint_receiver): (
                DebugEnvironment,
                MpscChannelReceiver<(DebugState, oneshot::Sender<DebugCommand>)>,
            ) = DebugEnvironment::new(breakpoints);

            // Spawn the debug task that will run the code in debug mode. It will send messages to the breakpoint channel whenever a breakpoint is hit.
            let outgoing_sender_clone = outgoing_sender.clone();
            let debug_task = tokio::spawn(async move {
                match runtime_env.run_debug_mode(&ast, &mut debug_env).await {
                    Ok(()) => {
                        let _ = outgoing_sender_clone.send(OutgoingDebuggerMessage::Finished);
                    }
                    Err(err) => {
                        let _ = outgoing_sender_clone
                            .clone()
                            .send(OutgoingDebuggerMessage::Error(format!(
                                "Debug session failed: {err}",
                            )));
                    }
                }
            });

            loop {
                tokio::select! {
                    breakpoint_hit = breakpoint_receiver.recv() => {
                        match breakpoint_hit {
                            Some((debug_state, command_sender)) => {
                                let _ = outgoing_sender.send(OutgoingDebuggerMessage::BreakpointHit(debug_state));

                                // Wait for the next debug command from the client
                                let send_res = match incoming_receiver.recv().await {
                                    Some(IncomingDebuggerMessage::StepInto) => {
                                        command_sender.send(DebugCommand::StepInto)
                                    }
                                    Some(IncomingDebuggerMessage::StepOver) => {
                                        command_sender.send(DebugCommand::StepOver)
                                    }
                                    Some(IncomingDebuggerMessage::Continue) => {
                                        command_sender.send(DebugCommand::Continue)
                                    }
                                    _ => {
                                        let _ = outgoing_sender.send(OutgoingDebuggerMessage::Error(
                                            "Expected StepInto, StepOver, or Continue message".to_string(),
                                        ));
                                        return;
                                    }
                                };

                                if let Err(_) = send_res {
                                    let _ = outgoing_sender.send(OutgoingDebuggerMessage::Error("Failed to advance debug session: debug command receiver was dropped".to_string()));
                                }
                            }
                            None => break, // Debug session ended, exit the loop
                        }
                    },
                    incoming_message = incoming_receiver.recv() => {
                        match incoming_message {
                            None => break, // All senders have been dropped, exit the loop
                            _ => {} // Ignore any incoming messages that are not expected at this point
                        }
                    },
                    else => break, // All senders have been dropped, exit the loop
                }
            }

            // If the loop has exited, it means the debug session has ended. We will close the debug task.
            debug_task.abort();
        });

        (incoming_sender, outgoing_receiver)
    }
}

#[cfg(test)]
mod tests {
    use crate::application::plugin::runtime::memory::{
        identifier::Identifier, values::VariableValue,
    };

    use super::*;
    use tokio::time::{Duration, timeout};

    async fn recv_message(
        receiver: &mut MpscChannelReceiver<OutgoingDebuggerMessage>,
    ) -> OutgoingDebuggerMessage {
        timeout(Duration::from_secs(2), receiver.recv())
            .await
            .expect("Timed out waiting for debugger output")
            .expect("Debugger channel closed unexpectedly")
    }

    #[tokio::test]
    async fn initializes_debugger_and_receives_finished_no_breakpoints() {
        let debugger = PluginDebugger::new();
        let (incoming_sender, mut outgoing_receiver) = debugger.debug().await;

        incoming_sender
            .send(IncomingDebuggerMessage::Initialize {
                code: "let x: int = 1 + 2;".to_string(),
                breakpoints: vec![],
            })
            .unwrap();

        match recv_message(&mut outgoing_receiver).await {
            OutgoingDebuggerMessage::Finished => {}
            OutgoingDebuggerMessage::Error(message) => {
                panic!("Did not expect runtime error, got: {message}");
            }
            _ => panic!("Expected Finished message from debugger"),
        }
    }

    #[tokio::test]
    async fn hits_breakpoint_and_continues_execution() {
        let debugger = PluginDebugger::new();
        let (incoming_sender, mut outgoing_receiver) = debugger.debug().await;

        incoming_sender
            .send(IncomingDebuggerMessage::Initialize {
                code: "let x: int = 1 + 2;\nlet y: int = x * 3;".to_string(),
                breakpoints: vec![1],
            })
            .unwrap();

        match recv_message(&mut outgoing_receiver).await {
            OutgoingDebuggerMessage::BreakpointHit(state) => {
                assert_eq!(state.variables.len(), 1);
                assert_eq!(state.variables[0].0, Identifier::new("x".to_string()));
                assert_eq!(state.variables[0].1, VariableValue::Int(3));
            }
            OutgoingDebuggerMessage::Error(message) => {
                panic!("Did not expect runtime error, got: {message}");
            }
            _ => panic!("Expected BreakpointHit message from debugger"),
        }

        incoming_sender
            .send(IncomingDebuggerMessage::Continue)
            .unwrap();

        match recv_message(&mut outgoing_receiver).await {
            OutgoingDebuggerMessage::Finished => {}
            OutgoingDebuggerMessage::Error(message) => {
                panic!("Did not expect runtime error after continue, got: {message}");
            }
            _ => panic!("Expected Finished message from debugger after continue"),
        }
    }

    #[tokio::test]
    async fn returns_error_when_initialize_message_is_missing() {
        let debugger = PluginDebugger::new();
        let (incoming_sender, mut outgoing_receiver) = debugger.debug().await;

        incoming_sender
            .send(IncomingDebuggerMessage::Continue)
            .unwrap();

        match recv_message(&mut outgoing_receiver).await {
            OutgoingDebuggerMessage::Error(message) => {
                assert!(message.contains("Expected Initialize message"));
            }
            _ => panic!("Expected an error message when initialize is missing"),
        }
    }

    #[tokio::test]
    async fn returns_error_when_source_code_cannot_be_parsed() {
        let debugger = PluginDebugger::new();
        let (incoming_sender, mut outgoing_receiver) = debugger.debug().await;

        incoming_sender
            .send(IncomingDebuggerMessage::Initialize {
                code: "let = ;".to_string(),
                breakpoints: vec![],
            })
            .unwrap();

        match recv_message(&mut outgoing_receiver).await {
            OutgoingDebuggerMessage::Error(message) => {
                assert!(message.contains("Failed to parse code"));
            }
            _ => panic!("Expected a parse error message"),
        }
    }

    #[tokio::test]
    async fn emits_finished_for_empty_program() {
        let debugger = PluginDebugger::new();
        let (incoming_sender, mut outgoing_receiver) = debugger.debug().await;

        incoming_sender
            .send(IncomingDebuggerMessage::Initialize {
                code: "".to_string(),
                breakpoints: vec![],
            })
            .unwrap();

        match recv_message(&mut outgoing_receiver).await {
            OutgoingDebuggerMessage::Finished => {}
            OutgoingDebuggerMessage::Error(message) => {
                panic!("Did not expect runtime error, got: {message}");
            }
            _ => panic!("Expected Finished message from debugger"),
        }
    }
}
