use uuid::Uuid;
use crate::domain::user::entities::User;

pub struct UserListProjection {
    pub id: Uuid,
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