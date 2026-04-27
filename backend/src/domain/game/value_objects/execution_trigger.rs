use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExecutionTrigger {
    BeforeAction(String),
    AfterAction(String),
    BeforeTurnAdvance,
    AfterTurnAdvance,
    BeforeRoundAdvance,
    AfterRoundAdvance,
}

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

impl ExecutionTrigger {
    pub fn parse_str(s: &str) -> Result<Self, String> {
        if s.starts_with("BeforeAction(") && s.ends_with(")") {
            let action = s[12..s.len() - 1].to_string();
            Ok(ExecutionTrigger::BeforeAction(action))
        } else if s.starts_with("AfterAction(") && s.ends_with(")") {
            let action = s[11..s.len() - 1].to_string();
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
