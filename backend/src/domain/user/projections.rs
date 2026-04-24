use crate::domain::{common::identifier::Identifier, user::entities::User};

pub struct UserListProjection {
    pub id: Identifier,
    pub name: String,
}

impl From<&User> for UserListProjection {
    fn from(user: &User) -> Self {
        Self {
            id: *user.id(),
            name: user.name().to_string(),
        }
    }
}
