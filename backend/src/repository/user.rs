use fnmock::derive::mock_function;
use sqlx::{query_as, SqlitePool};
use crate::entity::User;
use crate::error::RepositoryError;
use crate::{get_db_connection, map_query_err};

#[derive(Clone, PartialEq)]
pub struct UserCreateInformation {
    pub name: String,
    pub password: String,
}

pub async fn create_user(db: SqlitePool, user_info: UserCreateInformation) -> Result<User, RepositoryError> {
    if user_info.name.is_empty() || user_info.password.is_empty() {
        return Err(RepositoryError::InvalidParameter("Username or password must not be empty".to_string()));
    }

    let mut conn = get_db_connection!(db);

    let user: User = query_as!(
        User,
        "INSERT INTO users (name, password) VALUES ($1, $2) RETURNING id, name, password",
        user_info.name,
        user_info.password
    )
        .fetch_one(&mut *conn)
        .await
        .map_err(map_query_err!(|db_err| {
            if db_err.message().contains("UNIQUE constraint failed: users.name") {
                RepositoryError::Conflict("Username already exists".to_string())
            } else {
                RepositoryError::Database(db_err.message().to_string())
            }
        }))?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use crate::db::create_test_pool;
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let pool = create_test_pool().await;

        let user_create_info = UserCreateInformation {
            name: "Test".to_string(),
            password: "123456".to_string(),
        };

        let user = create_user(pool.clone(), user_create_info).await.unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Test");
        assert_eq!(user.password, "123456");

        let mut connection = pool.acquire().await.unwrap();
        let row = sqlx::query!("SELECT id, name, password FROM users")
            .fetch_one(&mut *connection).await.unwrap();

        assert_eq!(row.id, 1);
        assert_eq!(row.name, "Test");
        assert_eq!(user.password, "123456");
    }

    #[tokio::test]
    async fn test_create_user_with_empty_name() {
        let pool = create_test_pool().await;

        let user_create_info = UserCreateInformation {
            name: "".to_string(),
            password: "123456".to_string(),
        };

        let err = create_user(pool, user_create_info).await.unwrap_err();

        match err {
            RepositoryError::InvalidParameter(e) =>
                assert_eq!(e, "Username or password must not be empty".to_string()),
            _ => panic!("unexpected error"),
        }
    }

    #[tokio::test]
    async fn test_create_user_with_empty_password() {
        let pool = create_test_pool().await;

        let user_create_info = UserCreateInformation {
            name: "Test".to_string(),
            password: "".to_string(),
        };

        let err = create_user(pool, user_create_info).await.unwrap_err();

        match err {
            RepositoryError::InvalidParameter(e) =>
                assert_eq!(e, "Username or password must not be empty".to_string()),
            _ => panic!("unexpected error"),
        }
    }

    #[tokio::test]
    async fn test_create_user_with_duplicate_name() {
        let pool = create_test_pool().await;

        let user_create_info_1 = UserCreateInformation {
            name: "Test".to_string(),
            password: "123456".to_string(),
        };

        let _user_1 = create_user(pool.clone(), user_create_info_1).await.unwrap();

        let user_create_info_2 = UserCreateInformation {
            name: "Test".to_string(),
            password: "1234567".to_string(),
        };

        let err = create_user(pool, user_create_info_2).await.unwrap_err();
        
        match err {
            RepositoryError::Conflict(msg) => {
                assert_eq!(msg, "Username already exists");
            }
            _ => panic!("Expected Conflict error, got: {:?}", err),
        }
    }
}