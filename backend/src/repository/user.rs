use sqlx::pool::PoolConnection;
use sqlx::{query_as, Sqlite};
use crate::entity::User;

pub struct UserCreateInformation {
    pub name: String,
    pub password: String,
}

pub async fn create_user(mut connection: PoolConnection<Sqlite>, user_info: UserCreateInformation) -> Result<User, anyhow::Error> {
    let user: User = query_as!(
        User,
        "INSERT INTO users (name, password) VALUES ($1, $2) RETURNING id, name, password",
        user_info.name,
        user_info.password
    )
        .fetch_one(&mut *connection)
        .await?;

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

        let connection = pool.acquire().await.unwrap();
        let user = create_user(connection, user_create_info).await.unwrap();

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
}