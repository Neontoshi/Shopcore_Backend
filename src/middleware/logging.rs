use axum::{
    middleware::Next,
    response::Response,
    http::HeaderMap,
};
use axum::extract::Request;
use std::time::Instant;
use uuid::Uuid;

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_id = Uuid::new_v4().to_string();
    let user_agent = extract_header(request.headers(), "user-agent");
    let client_ip = extract_header(request.headers(), "x-forwarded-for")
        .or_else(|| extract_header(request.headers(), "x-real-ip"))
        .unwrap_or_else(|| "unknown".to_string());

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %uri.path(),
        client_ip = %client_ip,
        user_agent = %user_agent.unwrap_or_else(|| "unknown".to_string()),
        "incoming request"
    );

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %uri.path(),
        status = %status.as_u16(),
        duration_ms = %duration.as_millis(),
        "request completed"
    );

    response
}

fn extract_header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}