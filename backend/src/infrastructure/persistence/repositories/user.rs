use crate::application::user::contracts::UserRepositoryContract;
use crate::domain::common::identifier::Identifier;
use crate::domain::user::entities::User;
use crate::domain::user::error::{UserError, UserErrorKind};
use sqlx::{Acquire, SqlitePool};

struct UserRow {
    id: String,
    name: String,
    password: String,
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            password: user.password().to_string(),
        }
    }
}

impl TryInto<User> for UserRow {
    type Error = UserError;

    fn try_into(self) -> Result<User, Self::Error> {
        User::try_new(
            Identifier::parse_str(&self.id)
                .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?,
            self.name,
            self.password,
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

impl UserRepositoryContract for SqliteUserRepository {
    async fn check_if_exists(&self, id: &Identifier) -> Result<bool, UserError> {
        let mut conn = self
            .db
            .acquire()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        let id = id.to_string();
        let res = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        Ok(res.is_some())
    }

    async fn get_by_id(&self, id: &Identifier) -> Result<User, UserError> {
        let mut conn = self
            .db
            .acquire()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        let id = id.to_string();
        let res = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        if let Some(row) = res {
            row.try_into()
                .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))
        } else {
            Err(UserError::new(UserErrorKind::UserNotFound))
        }
    }

    async fn get_all(&self) -> Result<Vec<User>, UserError> {
        let mut conn = self
            .db
            .acquire()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        let res = sqlx::query_as!(UserRow, "SELECT * FROM users")
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        let users: Vec<User> = res
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<_, UserError>>()?;
        Ok(users)
    }

    async fn get_by_name(&self, name: &str) -> Result<User, UserError> {
        let mut conn = self
            .db
            .acquire()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        let res = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE name = $1", name)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        if let Some(row) = res {
            row.try_into()
                .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))
        } else {
            Err(UserError::new(UserErrorKind::UserNotFound))
        }
    }

    async fn save(&self, user: &User) -> Result<(), UserError> {
        let mut conn = self
            .db
            .acquire()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;
        let mut transaction = conn
            .begin()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        let name = user.name().to_string();
        let res = sqlx::query!("SELECT name FROM users WHERE name = $1", name)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        if let Some(_) = res {
            transaction
                .rollback()
                .await
                .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;
            return Err(UserError::new(UserErrorKind::UserAlreadyExists));
        }

        let user_insert_row = UserRow::from(user);
        sqlx::query_as!(
            UserRow,
            "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
            user_insert_row.id,
            user_insert_row.name,
            user_insert_row.password
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        transaction
            .commit()
            .await
            .map_err(|e| UserError::with_source(UserErrorKind::DatabaseError, Box::new(e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::user::contracts::UserRepositoryContract;
    use crate::infrastructure::persistence::db::create_test_pool;

    #[tokio::test]
    async fn test_save_and_get_by_id() {
        let uuid = Identifier::new();
        let user = User::try_new(
            uuid.clone(),
            "test-name".to_string(),
            "test-password".to_string(),
        )
        .unwrap();

        let repo = SqliteUserRepository::new(create_test_pool().await);

        let result = repo.save(&user).await;
        assert!(result.is_ok());

        let user = repo.get_by_id(&uuid).await.unwrap();
        assert_eq!(user, user);
    }

    #[tokio::test]
    async fn test_save_and_get_by_name() {
        let user = User::try_new(
            Identifier::new(),
            "test-name".to_string(),
            "test-password".to_string(),
        )
        .unwrap();

        let repo = SqliteUserRepository::new(create_test_pool().await);

        let result = repo.save(&user).await;
        assert!(result.is_ok());

        let user = repo.get_by_name("test-name").await.unwrap();
        assert_eq!(user, user);
    }

    #[tokio::test]
    async fn test_save_and_get_all() {
        let user = User::try_new(
            Identifier::new(),
            "test-name".to_string(),
            "test-password".to_string(),
        )
        .unwrap();
        let user2 = User::try_new(
            Identifier::new(),
            "test2-name".to_string(),
            "test2-password".to_string(),
        )
        .unwrap();

        let repo = SqliteUserRepository::new(create_test_pool().await);

        let result = repo.save(&user).await;
        assert!(result.is_ok());
        let result = repo.save(&user2).await;
        assert!(result.is_ok());

        let users = repo.get_all().await.unwrap();
        assert_eq!(users.len(), 2);
        assert!(users.contains(&user));
        assert!(users.contains(&user2));
    }

    #[tokio::test]
    async fn test_get_by_id_no_user() {
        let repo = SqliteUserRepository::new(create_test_pool().await);
        let id = Identifier::new();

        let user = repo.get_by_id(&id).await;

        assert!(user.is_err());
        let err = user.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::UserNotFound));
    }

    #[tokio::test]
    async fn test_get_by_name_no_user() {
        let repo = SqliteUserRepository::new(create_test_pool().await);
        let name = "test-name".to_string();

        let user = repo.get_by_name(&name).await;

        assert!(user.is_err());
        let err = user.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::UserNotFound));
    }

    #[tokio::test]
    async fn test_save_duplicate_name() {
        let repo = SqliteUserRepository::new(create_test_pool().await);
        let name = "test-name".to_string();

        let user1 =
            User::try_new(Identifier::new(), name.clone(), "test-password".to_string()).unwrap();
        let user2 = User::try_new(
            Identifier::new(),
            name.clone(),
            "test-password-2".to_string(),
        )
        .unwrap();

        let res = repo.save(&user1).await;
        assert!(res.is_ok());

        let res = repo.save(&user2).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::UserAlreadyExists));

        let res = repo.get_by_name(&name).await;
        assert!(res.is_ok());
        let user = res.unwrap();
        assert_eq!(user, user1);
    }
}
