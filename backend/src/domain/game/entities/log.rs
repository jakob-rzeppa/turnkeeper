use crate::domain::common::{date_time::DateTime, identifier::Identifier};

#[derive(Clone, Debug, PartialEq)]
pub struct Log {
    id: Identifier,

    entries: Vec<(LogEntry, DateTime)>,
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn log_action(&mut self, user_id: Identifier, action_id: Identifier, payload: String) {
        self.entries.push((
            LogEntry::Action {
                user_id,
                action_id,
                payload,
            },
            DateTime::now(),
        ));
    }

    pub fn log_system(&mut self, message: String) {
        self.entries
            .push((LogEntry::System { message }, DateTime::now()));
    }

    pub fn log_error(&mut self, message: String) {
        self.entries
            .push((LogEntry::Error { message }, DateTime::now()));
    }
}
