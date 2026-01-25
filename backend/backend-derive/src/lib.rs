use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro to automatically implement `axum::extract::FromRequest`
/// that extracts and validates JSON body.
///
/// # Example
///
/// ```rust
/// #[derive(serde::Deserialize, serde_valid::Validate, JsonRequest)]
/// pub struct MyHandlerRequest {
///     message: String,
/// }
/// ```
///
/// This will generate:
///
/// ```rust
/// impl<S> axum::extract::FromRequest<S> for MyHandlerRequest
/// where
///     S: Send + Sync,
/// {
///     type Rejection = crate::infrastructure::error::HttpError;
///
///     async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
///         // Check content type
///         let content_type = req
///             .headers()
///             .get(axum::http::header::CONTENT_TYPE)
///             .and_then(|v| v.to_str().ok())
///             .unwrap_or("");
///
///         if !content_type.starts_with("application/json") {
///             return Err(crate::infrastructure::error::HttpError::UnsupportedMediaType);
///         }
///
///         // Extract JSON
///         let extracted = match axum::Json::<Self>::from_request(req, state).await {
///             Ok(axum::Json(payload)) => payload,
///             Err(e) => return Err(crate::infrastructure::error::HttpError::BadRequest(e.to_string())),
///         };
///
///         // Validate
///         match extracted.validate() {
///             Ok(_) => Ok(extracted),
///             Err(e) => Err(crate::infrastructure::error::HttpError::BadRequest(e.to_string())),
///         }
///     }
/// }
/// ```
#[proc_macro_derive(JsonRequest)]
pub fn derive_json_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl<S> axum::extract::FromRequest<S> for #name
        where
            S: Send + Sync,
        {
            type Rejection = crate::infrastructure::error::HttpError;

            async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
                // Check content type
                let content_type = req
                    .headers()
                    .get(axum::http::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");

                if !content_type.starts_with("application/json") {
                    return Err(crate::infrastructure::error::HttpError::UnsupportedMediaType);
                }

                // Extract JSON
                let extracted = match axum::Json::<Self>::from_request(req, state).await {
                    Ok(axum::Json(payload)) => payload,
                    Err(e) => return Err(crate::infrastructure::error::HttpError::BadRequest(e.to_string())),
                };

                // Validate
                match extracted.validate() {
                    Ok(_) => Ok(extracted),
                    Err(e) => {
                        match crate::util::validation::convert_serde_valid_error(&e.to_string()) {
                            Ok(validation_errors) => Err(crate::infrastructure::error::HttpError::ValidationError(
                                format!("Validation failed: {0}", validation_errors),
                            )),
                            Err(e) => {
                                eprintln!("Parsing serde_valid error failed: {}", e);
                                Err(crate::infrastructure::error::HttpError::InternalServerError)
                            },
                        }
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro to automatically implement `axum::response::IntoResponse`
/// that returns a 200 OK status with JSON body.
///
/// # Example
///
/// ```rust
/// #[derive(serde::Serialize, JsonResponse)]
/// pub struct MyHandlerResponse {
///     message: String,
/// }
/// ```
///
/// This will generate:
///
/// ```rust
/// impl axum::response::IntoResponse for MyHandlerResponse {
///     fn into_response(self) -> axum::response::Response {
///         (axum::http::StatusCode::OK, axum::Json(self)).into_response()
///     }
/// }
/// ```
#[proc_macro_derive(JsonResponse)]
pub fn derive_json_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                (axum::http::StatusCode::OK, axum::Json(self)).into_response()
            }
        }
    };

    TokenStream::from(expanded)
}
