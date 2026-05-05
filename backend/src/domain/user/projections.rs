use crate::domain::{common::identifier::Id, user::entities::User};

pub struct UserListProjection {
    pub id: Id,
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
