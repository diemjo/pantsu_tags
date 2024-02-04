use axum::body::Body;
use axum::extract::Request;
use tower_http::request_id::RequestId;
use tracing::{error_span, Span};

pub fn request_id_tracing_span(request: &Request<Body>) -> Span {
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .map(|r| r.header_value().to_str().unwrap_or("invalid"))
        .unwrap_or_else(|| "unknown");
    error_span!(
        "request",
        id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
    )
}
