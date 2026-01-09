use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, PartialEq)]
pub struct User {
    pub id: i64,
    pub name: String,
    // the password is stored in plain text,
    // so the gm can look up a password if a user forgot it
    pub password: String,
}