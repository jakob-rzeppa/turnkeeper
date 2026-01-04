
/// Gets the db connection from the db pool
///
/// if a error occurred it returns a HttpError::InternalServerError
#[macro_export] macro_rules! get_db_connection_from_pool {
    ($db_pool: expr) => {
        match $db_pool.acquire().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to acquire DB connection: {}", e);
                return Err(crate::error::HttpError::InternalServerError)
            }
        }
    };
}

/// Maps SQLite errors to RepositoryError
///
/// Takes a closure that handles database-specific error conditions
#[macro_export]
macro_rules! map_query_err {
    ($custom_handler:expr) => {
        |e| {
            match e {
                sqlx::Error::Database(db_err) => {
                    let handler: fn(&dyn sqlx::error::DatabaseError) -> crate::error::RepositoryError = $custom_handler;
                    handler(db_err.as_ref())
                }
                _ => {
                    crate::error::RepositoryError::Database(e.to_string())
                }
            }
        }
    };
}