use sqlx::SqlitePool;
use uuid::Uuid;
use crate::domain::entity::user::User;
use crate::domain::error::Error;
use crate::domain::repository::UserRepositoryTrait;

struct SqliteUserRepository {
    db: SqlitePool,
}

impl UserRepositoryTrait for SqliteUserRepository {
    async fn get_by_id(&self, user_id: Uuid) -> Result<User, Error> {
        Err(Error::NotImplemented)
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }

    async fn update(&self, user: User) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }

    async fn delete(&self, user_id: Uuid) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}