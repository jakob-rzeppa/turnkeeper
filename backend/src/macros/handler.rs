/// Gets the db connection from the db pool
///
/// if a error occurred it returns a HttpError::InternalServerError
#[macro_export] macro_rules! get_db_connection {
    ($db_pool: expr) => {
        match $db_pool.acquire().await {
            Ok(conn) => conn,
            Err(e) => {
                return Err(crate::domain::error::Error::DatabaseError { msg: format!("failed to acquire DB connection: {}", e) })
            }
        }
    };
}