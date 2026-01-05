
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