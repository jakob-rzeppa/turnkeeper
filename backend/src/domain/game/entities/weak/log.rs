use crate::domain::common::{date_time::DateTime, identifier::Identifier};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Log {
    entries: Vec<(LogEntry, DateTime)>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LogEntry {
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
            entries: Vec::new(),
        }
    }

    pub fn new_raw(entries: Vec<(LogEntry, DateTime)>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &Vec<(LogEntry, DateTime)> {
        &self.entries
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
