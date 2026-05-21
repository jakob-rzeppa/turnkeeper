use tokio::task::JoinHandle;

use crate::{
    application::{
        action_interpreter::{ ActionExecutor, debug_env::{ DebugCommand, DebugEnvironment } },
        common::{
            channels::mpsc::{ MpscChannel, MpscChannelReceiver, MpscChannelSender },
            parser::{ GameParser, GameParserContract },
        },
        game::debugger::{ commands::DebuggerCommand, message::DebuggerMessage },
    },
    domain::{ common::identifier::Id, game::entities::{ game::Game, game_instance::GameInstance } },
};

pub mod commands;
pub mod message;

pub struct DebuggerSession {
    /// The task handle for the session task.
    task_handle: Option<JoinHandle<()>>,
}

impl DebuggerSession {
    /// Spawns a new debugger session for the given game. The session will run in a separate task.
    /// The session will continue running until the DebuggerSession is dropped, at which point the task will be aborted.
    pub fn spawn(
        game: Game,
        user_id: Id
    ) -> (Self, MpscChannelSender<DebuggerCommand>, MpscChannelReceiver<DebuggerMessage>) {
        let (command_sender, mut command_receiver) = MpscChannel::<DebuggerCommand>::new();
        let (message_sender, message_receiver) = MpscChannel::<DebuggerMessage>::new();

        let task_handle = tokio::spawn(async move {
            // Expect a setup command, else ignore
            let mut setup_info = None;
            while setup_info.is_none() {
                setup_info = match command_receiver.recv().await {
                    Some(DebuggerCommand::Setup { stat_values, players }) =>
                        Some((stat_values, players)),
                    Some(_) => {
                        continue;
                    } // Ignore any other commands until we get a Setup command
                    None => {
                        break; // Channel closed, exit the loop
                    }
                };
            }

            let (stats, players) = match setup_info {
                Some(info) => info,
                None => {
                    // If we exit the loop without receiving a Setup command, just return and end the task
                    return;
                }
            };

            // Expect a start command, else ignore
            let mut start_info = None;
            while start_info.is_none() {
                start_info = match command_receiver.recv().await {
                    Some(DebuggerCommand::Start { action, params, breakpoints }) =>
                        Some((action, params, breakpoints)),
                    Some(_) => {
                        continue;
                    } // Ignore any other commands until we get a Start command
                    None => {
                        break; // Channel closed, exit the loop
                    }
                };
            }

            let (action, params, breakpoints) = match start_info {
                Some(info) => info,
                None => {
                    // If we exit the loop without receiving a Start command, just return and end the task
                    return;
                }
            };

            // Parse the game
            let game_parser = GameParser::new();

            let game_parsing_result = match game_parser.parse_game(game.source_code()) {
                Ok(result) => result,
                Err(e) => {
                    _ = message_sender.send(DebuggerMessage::ParsingError {
                        message: e.to_string(),
                    });
                    return; // Stop debugger if parsing fails
                }
            };

            let mut game_instance = GameInstance::new(
                "Debugger Game Instance".to_string(), // For now, we can just use a placeholder name for the game instance
                user_id,
                game_parsing_result.game_stats,
                game_parsing_result.player_stats,
                game_parsing_result.actions,
                game_parsing_result.pages,
                game.clone()
            );

            // Set up the game instance with the provided stat values and player info
            for (stat_name, value) in stats {
                match game_instance.set_game_stat_value(&stat_name, value) {
                    Ok(()) => (),
                    Err(e) => {
                        _ = message_sender.send(DebuggerMessage::SetupError {
                            message: e.to_string(),
                        });
                        return;
                    }
                }
            }

            for (player_name, player_stats) in players {
                let initial_player_name = match game_instance.add_player() {
                    Ok(name) => name,
                    Err(e) => {
                        _ = message_sender.send(DebuggerMessage::SetupError {
                            message: e.to_string(),
                        });
                        return;
                    }
                };
                match game_instance.change_player_name(initial_player_name, player_name.clone()) {
                    Ok(()) => (),
                    Err(e) => {
                        _ = message_sender.send(DebuggerMessage::SetupError {
                            message: e.to_string(),
                        });
                        return;
                    }
                }
                for (stat_name, value) in player_stats {
                    match game_instance.set_player_stat_value(&player_name, &stat_name, value) {
                        Ok(()) => (),
                        Err(e) => {
                            _ = message_sender.send(DebuggerMessage::SetupError {
                                message: e.to_string(),
                            });
                            return;
                        }
                    }
                }
            }

            // Set up the debug environment and executor
            let (mut debug_env, mut breakpoint_channel_receiver) = DebugEnvironment::new(
                breakpoints
                    .into_iter()
                    .map(|b| usize::try_from(b))
                    .collect::<Result<Vec<usize>, _>>()
                    .unwrap_or(Vec::new()) // If any breakpoint cannot be converted to usize, ignore all breakpoints
            );

            let executor = ActionExecutor::new(game_instance, &action, user_id).unwrap();

            // Start executing the action in a separate task so that we can listen for breakpoint hits concurrently
            let debug_task_handle = tokio::spawn(async move {
                executor.execute_debug(params, &mut debug_env).await
            });

            // Handle breakpoint hits
            loop {
                tokio::select! {
                    msg = breakpoint_channel_receiver.recv() => {
                        match msg {
                            Some((projection, debug_command_sender)) => {
                                // Send a message to the frontend with the current projection and a new command sender for the session to listen for commands related to this breakpoint hit
                                let _ = message_sender.send(DebuggerMessage::BreakpointHit {
                                    environment: projection,
                                });

                                // Wait for a command from the frontend related to this breakpoint hit, in this time the breakpoint channel will not receive any new breakpoint hits
                                match command_receiver.recv().await {
                                    Some(DebuggerCommand::StepInto) => {
                                        let _ = debug_command_sender.send(DebugCommand::StepInto);
                                    }
                                    Some(DebuggerCommand::StepOver) => {
                                        let _ = debug_command_sender.send(DebugCommand::StepOver);
                                    }
                                    Some(DebuggerCommand::Continue) => {
                                        let _ = debug_command_sender.send(DebugCommand::Continue);
                                    }
                                    Some(_) => {
                                        // Ignore any other commands while waiting for a breakpoint command
                                        continue;
                                    }
                                    None => {
                                        break; // Channel closed, exit the loop
                                    }
                                }
                            }
                            None => {
                                break; // If the channel is closed, exit the loop
                            }
                        }
                    }
                    msg = command_receiver.recv() => {
                        match msg {
                            Some(_) => {
                                // Ignore any commands that come in while we're not waiting for a breakpoint command
                                continue;
                            }
                            None => {
                                break; // Channel closed, exit the loop 
                            }
                        }
                    }
                }
            }

            // Since the loop before will close, when the executor finishes, since the channel will be closed, we can just break the loop and then wait for the executor result to send the final message
            match debug_task_handle.await {
                Ok(result) => {
                    match result {
                        Ok(projection) => {
                            _ = message_sender.send(DebuggerMessage::Finished {
                                environment: projection,
                            });
                        }
                        Err(e) => {
                            _ = message_sender.send(DebuggerMessage::RuntimeError {
                                message: e.to_string(),
                            });
                        }
                    }
                }
                Err(e) => {
                    _ = message_sender.send(DebuggerMessage::InternalError {
                        message: e.to_string(),
                    });
                }
            };
        });

        (
            Self {
                task_handle: Some(task_handle),
            },
            command_sender,
            message_receiver,
        )
    }
}

impl Drop for DebuggerSession {
    /// When the DebuggerSession is dropped, we want to ensure that the associated task is also stopped.
    fn drop(&mut self) {
        if let Some(handle) = self.task_handle.take() {
            handle.abort();
        }
    }
}
