use sqlx::{Acquire, SqlitePool};
use uuid::Uuid;
use crate::domain::auth::entities::User;
use crate::application::auth::traits::UserRepositoryTrait;
use crate::domain::error::Error;

struct UserRow {
    id: String,
    name: String,
    password: String,
}

impl From<User> for UserRow {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            password: user.password().to_string(),
        }
    }
}

impl TryInto<User> for UserRow {
    type Error = Error;

    fn try_into(self) -> Result<User, Self::Error> {
        let id = Uuid::try_from(self.id).map_err(|_| Error::DatabaseError { msg: "Could not convert db Uuid to Uuid".to_string() })?;
        User::try_new(
            id,
            self.name.clone(),
            self.password.clone(),
        )
    }
}

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
        let mut conn = self.db.acquire().await.map_err(|_| Error::DatabaseError { msg: "Error acquiring connection".into() })?;

        let id = id.to_string();
        let res = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|_| Error::DatabaseError { msg: "Unexpected database error".into() })?;

        if let Some(row) = res {
            row.try_into().map_err(|_| Error::DatabaseError { msg: "Invalid db state".into() })
        } else {
            Err(Error::NotFound { msg: format!("User with id {} not found", id) })
        }
    }

    async fn get_by_name(&self, name: String) -> Result<User, Error> {
        let mut conn = self.db.acquire().await.map_err(|_| Error::DatabaseError { msg: "Error acquiring connection".into() })?;

        let res = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE name = $1", name)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|_| Error::DatabaseError { msg: "Unexpected database error".into() })?;

        if let Some(row) = res {
            row.try_into().map_err(|_| Error::DatabaseError { msg: "Invalid db state".into() })
        } else {
            Err(Error::NotFound { msg: format!("User with name {} not found", name) })
        }
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        let mut conn = self.db.acquire().await.map_err(|_| Error::DatabaseError { msg: "Error acquiring connection".into() })?;
        let mut transaction = conn.begin().await.map_err(|_| Error::DatabaseError { msg: "Error starting transaction".into() })?;

        let name = user.name().to_string();
        let res = sqlx::query!("SELECT name FROM users WHERE name = $1", name)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| Error::DatabaseError { msg: "Unexpected database error".into() })?;

        if let Some(row) = res {
            transaction.rollback().await.map_err(|_| Error::DatabaseError { msg: "Error committing transaction".into() })?;
            return Err(Error::InvalidState { msg: format!("User with name {} already exists", user.name())});
        }

        let user_insert_row: UserRow = UserRow::from(user);
        let res = sqlx::query_as!(
            UserRow,
            "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
            user_insert_row.id, user_insert_row.name, user_insert_row.password
        )
            .execute(&mut *transaction)
            .await
            .map_err(|_| Error::DatabaseError { msg: "Unexpected database error".into() })?;

        transaction.commit().await.map_err(|_| Error::DatabaseError { msg: "Error committing transaction".into() })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::domain::auth::entities::User;
    use crate::domain::error::Error;
    use crate::application::auth::traits::UserRepositoryTrait;
    use crate::infrastructure::db::create_test_pool;
    use crate::infrastructure::repository::user::SqliteUserRepository;

    #[tokio::test]
    async fn test_save_and_get_by_id() {
        let uuid = Uuid::new_v4();
        let user = User::try_new(
            uuid.clone(),
            "test-name".to_string(),
            "test-password".to_string(),
        ).unwrap();

        let repo = SqliteUserRepository::new(create_test_pool().await);

        let result = repo.save(user).await;
        assert!(result.is_ok());

        let user = repo.get_by_id(uuid).await.unwrap();
        assert_eq!(user, user);
    }

    #[tokio::test]
    async fn test_save_and_get_by_name() {
        let user = User::try_new(
            Uuid::new_v4(),
            "test-name".to_string(),
            "test-password".to_string(),
        ).unwrap();

        let repo = SqliteUserRepository::new(create_test_pool().await);

        let result = repo.save(user).await;
        assert!(result.is_ok());

        let user = repo.get_by_name("test-name".to_string()).await.unwrap();
        assert_eq!(user, user);
    }

    #[tokio::test]
    async fn test_get_by_id_no_user() {
        let repo = SqliteUserRepository::new(create_test_pool().await);
        let id = Uuid::new_v4();

        let user = repo.get_by_id(id.clone()).await;

        assert!(user.is_err());
        let err = user.unwrap_err();
        assert_eq!(err, Error::NotFound { msg: format!("User with id {} not found", id) });
    }

    #[tokio::test]
    async fn test_get_by_name_no_user() {
        let repo = SqliteUserRepository::new(create_test_pool().await);
        let name = "test-name".to_string();

        let user = repo.get_by_name(name.clone()).await;

        assert!(user.is_err());
        let err = user.unwrap_err();
        assert_eq!(err, Error::NotFound { msg: format!("User with name {} not found", name) });
    }

    #[tokio::test]
    async fn test_save_duplicate_name() {
        let repo = SqliteUserRepository::new(create_test_pool().await);
        let name = "test-name".to_string();

        let user1 = User::try_new(
            Uuid::new_v4(),
            name.clone(),
            "test-password".to_string(),
        ).unwrap();
        let user2 = User::try_new(
            Uuid::new_v4(),
            name.clone(),
            "test-password-2".to_string(),
        ).unwrap();

        let res = repo.save(user1.clone()).await;
        assert!(res.is_ok());

        let res = repo.save(user2).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, Error::InvalidState { msg: format!("User with name {} already exists", name) });

        let res = repo.get_by_name(name.clone()).await;
        assert!(res.is_ok());
        let user = res.unwrap();
        assert_eq!(user, user1);
    }
}