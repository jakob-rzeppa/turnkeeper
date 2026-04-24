use crate::domain::common::{date_time::DateTime, identifier::Identifier};

pub struct Log {
    id: Identifier,

    entries: Vec<(LogEntry, DateTime)>,
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
