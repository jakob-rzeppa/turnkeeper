/// Gets the db connection from the db pool
///
/// if a error occurred it returns a HttpError::InternalServerError
#[macro_export] macro_rules! get_db_connection {
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

/// Implements .into_response for the struct with a StatusCode::Ok
#[macro_export] macro_rules! json_response {
    ($struct_name:ident, $struct:tt) => {
        #[derive(Serialize)]
        pub struct $struct_name $struct

        impl IntoResponse for $struct_name {
            fn into_response(self) -> axum::response::Response {
                (StatusCode::OK, Json(self)).into_response()
            }
        }
    };
}