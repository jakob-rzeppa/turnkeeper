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

/// Creates JSON request and response structs for a handler
///
/// USAGE:
///
/// json_request!('name', 'request definition', 'response definition')
///
/// After, the 'name' + Request and 'name' + Response structs can be used.
///
/// The Request struct implements FromRequest and the Response IntoResponse,
/// so they can be used directly in axum handlers.
///
/// EXAMPLE:
///
/// json_request!(GetUser, { id: i64 }, { id: i64, name: String, ... });
///
/// async fn get_user(request: GetUserRequest) -> Result<GetUserResponse, HttpError> { ... }
///
#[macro_export] 
macro_rules! json_handler {
    ($handler_name:ident, $request:tt, $response:tt) => {
        paste::paste! {
            #[derive(serde::Deserialize, Debug)]
            pub struct [<$handler_name Request>] $request

            impl<S> axum::extract::FromRequest<S> for [<$handler_name Request>]
            where
                S: Send + Sync,
            {
                type Rejection = crate::error::HttpError;

                async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
                    // Check content type
                    let content_type = req
                        .headers()
                        .get(axum::http::header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");

                    if !content_type.starts_with("application/json") {
                        return Err(crate::error::HttpError::UnsupportedMediaType);
                    }

                    // Extract JSON
                    match axum::Json::<Self>::from_request(req, state).await {
                        Ok(axum::Json(payload)) => Ok(payload),
                        Err(e) => Err(crate::error::HttpError::BadRequest(e.to_string())),
                    }
                }
            }

            #[derive(serde::Serialize, Debug)]
            pub struct [<$handler_name Response>] $response

            impl axum::response::IntoResponse for [<$handler_name Response>] {
                fn into_response(self) -> axum::response::Response {
                    (axum::http::StatusCode::OK, axum::Json(self)).into_response()
                }
            }
        }
    };
}