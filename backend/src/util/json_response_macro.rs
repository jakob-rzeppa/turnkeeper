
/// Implements .into_response for the struct with a StatusCode::Ok
#[macro_export] macro_rules! json_response {
    ($struct_name:ident) => {
        impl IntoResponse for $struct_name {
            fn into_response(self) -> axum::response::Response {
                (StatusCode::OK, Json(self)).into_response()
            }
        }
    };
}