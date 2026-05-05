use std::fmt::Display;

use crate::{
    application::game_instance::commands::GameSessionCommand,
    domain::{
        common::identifier::Id,
        game::projections::{
            game_display_template::GameDisplayTemplateProjection,
            game_instance_state::GameInstanceStateProjection,
        },
    },
};

#[derive(Debug)]
pub enum IncomingMessageDto {
    Command {
        command: GameSessionCommand,
        sending_user_id: Id,
    },
}

#[derive(Debug, Clone)]
pub enum OutgoingMessageDto {
    DisplayTemplate(GameDisplayTemplateProjection),
    State(GameInstanceStateProjection),
    Error(String),
}

impl Display for OutgoingMessageDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutgoingMessageDto::DisplayTemplate(info) => {
                write!(
                    f,
                    "DisplayTemplate {}",
                    serde_json::to_string(info).unwrap()
                )
            }
            OutgoingMessageDto::State(info) => {
                write!(f, "State {}", serde_json::to_string(info).unwrap())
            }
            OutgoingMessageDto::Error(error) => {
                write!(f, "Error {}", error)
            }
        }
    }
}
