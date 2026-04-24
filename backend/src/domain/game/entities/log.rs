use chrono::{DateTime, Local};

use crate::domain::common::identifier::Identifier;

pub struct Log {
    id: Identifier,

    entries: Vec<(LogEntry, DateTime<Local>)>,
}

enum LogEntry {
    Action {
        user_id: Identifier,
        action_id: Identifier,
        payload: String,
    },
    System {
        message: String,
    },
    Error {
        message: String,
    },
}

impl Log {
    pub fn new() -> Self {
        Self {
            id: Identifier::new(),
            entries: Vec::new(),
        }
    }
}
