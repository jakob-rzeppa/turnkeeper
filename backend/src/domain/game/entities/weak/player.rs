use crate::domain::common::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    name: String,
    user_id: Option<Identifier>,
}

impl Player {
    /// Creates a new anonymous player with no linked user and no stats.
    pub fn new() -> Self {
        Self {
            name: format!("Anonymous Player {0}", Identifier::new().to_string()), // For now, we can generate a random name using an identifier. In the future, we might add a name generator to add some variety to the names. The name can be changed later.
            user_id: None,
        }
    }

    pub fn new_raw(name: String, user_id: Option<Identifier>) -> Self {
        Self { name, user_id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn user_id(&self) -> Option<&Identifier> {
        self.user_id.as_ref()
    }

    pub fn attach_user(&mut self, user_id: Identifier) {
        self.user_id = Some(user_id);
    }

    pub fn detach_user(&mut self) {
        self.user_id = None;
    }
}
