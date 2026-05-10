use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    ExprMethodCall,
    FnArg,
    ImplItemFn,
    PatType,
    Token,
    parse_macro_input,
    parse_quote,
    punctuated::Punctuated,
    visit_mut::VisitMut,
    ItemImpl,
};

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

    let expanded =
        quote! {
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
                match axum::Json::<Self>::from_request(req, state).await {
                    Ok(axum::Json(payload)) => Ok(payload),
                    Err(e) => Err(crate::infrastructure::error::HttpError::BadRequest(e.to_string())),
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

    let expanded =
        quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                (axum::http::StatusCode::OK, axum::Json(self)).into_response()
            }
        }
    };

    TokenStream::from(expanded)
}

struct ExecuteToDebugRewriter;

impl VisitMut for ExecuteToDebugRewriter {
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        syn::visit_mut::visit_expr_method_call_mut(self, node);

        if node.method == "execute" {
            node.method = syn::Ident::new("execute_debug", node.method.span());
            node.args.push(parse_quote!(debug_env));
        }
    }
}

#[proc_macro_attribute]
pub fn execute_debug(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let method = parse_macro_input!(input as ImplItemFn);

    if method.sig.ident != "execute" {
        return syn::Error
            ::new_spanned(
                &method.sig.ident,
                "#[execute_debug] can only be used on a method named execute"
            )
            .to_compile_error()
            .into();
    }

    if method.sig.asyncness.is_none() {
        return syn::Error
            ::new_spanned(&method.sig.ident, "#[execute_debug] can only be used on async methods")
            .to_compile_error()
            .into();
    }

    let mut debug_method = method.clone();
    debug_method.sig.ident = syn::Ident::new("execute_debug", debug_method.sig.ident.span());

    let debug_arg: FnArg =
        parse_quote!(
        debug_env: &mut crate::application::plugin::runtime::debug::DebugEnvironment
    );

    let mut new_inputs: Punctuated<FnArg, Token![,]> = Punctuated::new();
    let mut inserted = false;
    for arg in &method.sig.inputs {
        new_inputs.push(arg.clone());

        if
            let FnArg::Typed(PatType { pat, .. }) = arg &&
            let syn::Pat::Ident(pat_ident) = pat.as_ref() &&
            pat_ident.ident == "env"
        {
            new_inputs.push(debug_arg.clone());
            inserted = true;
        }
    }

    if !inserted {
        return syn::Error
            ::new_spanned(
                &method.sig.ident,
                "#[execute_debug] expected an `env` argument to place `debug_env` next to it"
            )
            .to_compile_error()
            .into();
    }

    debug_method.sig.inputs = new_inputs;

    let mut rewritten_block = method.block.clone();
    ExecuteToDebugRewriter.visit_block_mut(&mut rewritten_block);

    debug_method.block =
        parse_quote!({
        let stepping_over = debug_env
            .wait(
                crate::application::plugin::parser::abstract_syntax_tree::Positioned::position(self)
                    .line(),
                env.get_debug_state(),
            )
            .await;

        // Ensure step-over finishes even when execution returns with an error.
        let mut internal_fn = async || #rewritten_block;
        let res = internal_fn().await;

        if stepping_over {
            debug_env.finish_step_over();
        }

        res
    });

    TokenStream::from(quote! {
        #method
        #debug_method
    })
}

/// Attribute macro to automatically implement `serde::Serialize` based on a `Display` implementation.
///
/// Place this attribute before an `impl Display` block. It will generate the corresponding
/// `impl Serialize` that serializes using the Display implementation.
///
/// # Example
///
/// ```rust
/// #[serialize_use_display]
/// impl Display for ExecutionTrigger {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         // ... implementation
///     }
/// }
/// ```
///
/// This will generate:
///
/// ```rust
/// impl Serialize for ExecutionTrigger {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: serde::Serializer,
///     {
///         serializer.serialize_str(&self.to_string())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn serialize_use_display(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(input as ItemImpl);

    let target_type = &impl_block.self_ty;
    let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();

    let serialize_impl =
        quote! {
        impl #impl_generics serde::Serialize for #target_type #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
    };

    let expanded = quote! {
        #impl_block
        #serialize_impl
    };

    TokenStream::from(expanded)
}

/// Attribute macro to automatically implement `serde::Deserialize` based on a `FromStr` implementation.
///
/// Place this attribute before an `impl FromStr` block. It will generate the corresponding
/// `impl Deserialize` that deserializes using the FromStr implementation.
///
/// # Example
///
/// ```rust
/// #[deserialize_use_from_str]
/// impl FromStr for ExecutionTrigger {
///     type Err = String;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         // ... implementation
///     }
/// }
/// ```
///
/// This will generate:
///
/// ```rust
/// impl<'de> Deserialize<'de> for ExecutionTrigger {
///     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
///     where
///         D: serde::Deserializer<'de>,
///     {
///         let s = String::deserialize(deserializer)?;
///         ExecutionTrigger::from_str(&s).map_err(serde::de::Error::custom)
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn deserialize_use_from_str(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(input as ItemImpl);

    let target_type = &impl_block.self_ty;
    let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();

    let deserialize_impl =
        quote! {
        impl #impl_generics<'de> serde::Deserialize<'de> for #target_type #ty_generics #where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)
            }
        }
    };

    let expanded = quote! {
        #impl_block
        #deserialize_impl
    };

    TokenStream::from(expanded)
}
