
/// Gets the db connection from the db pool
///
/// if a error occurred it returns a HttpError::InternalServerError
#[macro_export] macro_rules! get_db_connection_from_pool {
    ($db_pool: expr) => {
        match $db_pool.acquire().await {
            Ok(conn) => conn,
            Err(e) => {
                return Err(HttpError::InternalServerError(format!("Couldn't get db pool connection: {}", e)))
            }
        }
    };
}