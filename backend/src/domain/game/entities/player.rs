use crate::domain::common::identifier::Identifier;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    id: Identifier,
    user_id: Option<Identifier>,
}

impl Player {
    /// Creates a new anonymous player with no linked user and no stats.
    pub fn new(id: Identifier) -> Self {
        Self { id, user_id: None }
    }

    pub fn attach_user(&mut self, user_id: Identifier) {
        self.user_id = Some(user_id);
    }

    pub fn detach_user(&mut self) {
        self.user_id = None;
    }
}
