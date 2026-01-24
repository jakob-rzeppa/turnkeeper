use fnmock::derive::mock_function;
use sqlx::{query, query_as, SqlitePool};
use crate::entity::User;
use crate::error::RepositoryError;
use crate::{get_db_connection, map_query_err};

#[mock_function(ignore = [db])]
pub async fn get_user(db: SqlitePool, id: i64) -> Result<Option<User>, RepositoryError> {
    let mut conn = get_db_connection!(db);

    let user: Option<User> = query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        id
    )
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_query_err!(|db_err| {
            RepositoryError::Database(db_err.message().to_string())
        }))?;

    Ok(user)
}

#[mock_function(ignore = [db])]
pub async fn get_id_by_name_if_password(db: SqlitePool, name: String, password: String) -> Result<Option<i64>, RepositoryError> {
    let mut conn = get_db_connection!(db);

    let result = query!(
        "SELECT id FROM users WHERE name = $1 AND password = $2",
        name,
        password
    )
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_query_err!(|db_err| {
            RepositoryError::Database(db_err.message().to_string())
        }))?
        .and_then(|row| row.id);

    Ok(result)
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserCreateInformation {
    pub name: String,
    pub password: String,
}

#[mock_function(ignore = [db])]
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
    use sqlx::query;
    use crate::infrastructure::db::create_test_pool;
    use super::*;

    #[tokio::test]
    async fn test_get_user() {
        let pool = create_test_pool().await;

        let id = 1;

        {
            let mut connection = pool.acquire().await.unwrap();
            query!(
                "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
                id,
                "some name",
                "some password"
            )
                .execute(&mut *connection).await.unwrap();
        }

        let user = get_user(pool, id).await.unwrap();

        assert!(user.is_some());
        assert_eq!(user.unwrap(), User {
            id: 1,
            name: "some name".to_string(),
            password: "some password".to_string(),
        });
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let pool = create_test_pool().await;

        {
            let mut connection = pool.acquire().await.unwrap();
            query!(
                "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
                1,
                "some name",
                "some password"
            )
                .execute(&mut *connection).await.unwrap();
        }

        let user = get_user(pool, 999).await.unwrap();

        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_get_id_by_name_if_password() {
        let pool = create_test_pool().await;

        let id = 1;

        {
            let mut connection = pool.acquire().await.unwrap();
            query!(
                "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
                id,
                "some name",
                "some password"
            )
                .execute(&mut *connection).await.unwrap();
        }

        let user = get_id_by_name_if_password(pool, "some name".to_string(), "some password".to_string()).await.unwrap();

        assert!(user.is_some());
        assert_eq!(user.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_id_by_name_if_password_invalid_password() {
        let pool = create_test_pool().await;

        {
            let mut connection = pool.acquire().await.unwrap();
            query!(
                "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
                1,
                "some name",
                "some password"
            )
                .execute(&mut *connection).await.unwrap();
        }

        let user = get_id_by_name_if_password(pool, "some name".to_string(), "invalid password".to_string()).await.unwrap();

        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_get_id_by_name_if_password_invalid_name() {
        let pool = create_test_pool().await;

        {
            let mut connection = pool.acquire().await.unwrap();
            query!(
                "INSERT INTO users (id, name, password) VALUES ($1, $2, $3)",
                1,
                "some name",
                "some password"
            )
                .execute(&mut *connection).await.unwrap();
        }

        let user = get_id_by_name_if_password(pool, "invalid name".to_string(), "some password".to_string()).await.unwrap();

        assert!(user.is_none());
    }

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