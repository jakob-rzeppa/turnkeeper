use std::collections::HashMap;

use crate::domain::game::value_objects::data::Value;

pub enum DebuggerCommand {
    Setup {
        stat_values: HashMap<String, Value>,
        players: Vec<(String, HashMap<String, Value>)>, // Players in order with their stat values
    },
    Start {
        action: String,
        params: HashMap<String, Value>,
        breakpoints: Vec<u32>,
    },
    StepInto,
    StepOver,
    Continue,
}
