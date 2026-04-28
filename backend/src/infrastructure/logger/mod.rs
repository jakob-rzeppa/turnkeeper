use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Middleware that logs incoming requests.
pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let origin = req
        .headers()
        .get("origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown")
        .to_string();

    let response = next.run(req).await;

    // Log the outgoing response
    println!("{}: {} {} -> {}", origin, method, uri, response.status());

    response
}
