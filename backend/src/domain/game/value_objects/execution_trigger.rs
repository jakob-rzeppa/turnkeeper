use std::{fmt::Display, str::FromStr};

use backend_derive::{serialize_use_display, deserialize_use_from_str};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionTrigger {
    BeforeAction(String),
    AfterAction(String),
    BeforeTurnAdvance,
    AfterTurnAdvance,
    BeforeRoundAdvance,
    AfterRoundAdvance,
}

#[serialize_use_display]
impl Display for ExecutionTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionTrigger::BeforeAction(action) => write!(f, "BeforeAction({})", action),
            ExecutionTrigger::AfterAction(action) => write!(f, "AfterAction({})", action),
            ExecutionTrigger::BeforeTurnAdvance => write!(f, "BeforeTurnAdvance"),
            ExecutionTrigger::AfterTurnAdvance => write!(f, "AfterTurnAdvance"),
            ExecutionTrigger::BeforeRoundAdvance => write!(f, "BeforeRoundAdvance"),
            ExecutionTrigger::AfterRoundAdvance => write!(f, "AfterRoundAdvance"),
        }
    }
}

#[deserialize_use_from_str]
impl FromStr for ExecutionTrigger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("BeforeAction(") && s.ends_with(")") {
            let action = s[13..s.len() - 1].to_string();
            Ok(ExecutionTrigger::BeforeAction(action))
        } else if s.starts_with("AfterAction(") && s.ends_with(")") {
            let action = s[12..s.len() - 1].to_string();
            Ok(ExecutionTrigger::AfterAction(action))
        } else if s == "BeforeTurnAdvance" {
            Ok(ExecutionTrigger::BeforeTurnAdvance)
        } else if s == "AfterTurnAdvance" {
            Ok(ExecutionTrigger::AfterTurnAdvance)
        } else if s == "BeforeRoundAdvance" {
            Ok(ExecutionTrigger::BeforeRoundAdvance)
        } else if s == "AfterRoundAdvance" {
            Ok(ExecutionTrigger::AfterRoundAdvance)
        } else {
            Err(format!("Invalid ExecutionTrigger string: {}", s))
        }
    }
}
