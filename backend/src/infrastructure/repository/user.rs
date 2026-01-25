use sqlx::SqlitePool;
use uuid::Uuid;
use crate::domain::entity::user::User;
use crate::domain::error::Error;
use crate::domain::repository::UserRepositoryTrait;

pub struct SqliteUserRepository {
    db: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl UserRepositoryTrait for SqliteUserRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<User, Error> {
        Err(Error::NotImplemented)
    }

    async fn get_by_name(&self, name: String) -> Result<User, Error> {
        Err(Error::NotImplemented)
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}