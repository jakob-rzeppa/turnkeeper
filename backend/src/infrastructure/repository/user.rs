use sqlx::SqlitePool;
use crate::domain::entity::user::User;
use crate::domain::error::Error;
use crate::domain::repository::UserRepositoryTrait;

struct SqliteUserRepository {
    db: SqlitePool,
}

impl UserRepositoryTrait for SqliteUserRepository {
    async fn get_by_name(&self, name: String) -> Result<User, Error> {
        Err(Error::NotImplemented)
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}