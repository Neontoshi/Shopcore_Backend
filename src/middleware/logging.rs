use axum::{
    middleware::Next,
    response::Response,
    http::{HeaderMap, HeaderValue},
    extract::Request,
};
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
    
    // Get client IP from various headers
    let client_ip = request.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .or_else(|| request.headers()
            .get("x-real-ip")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string()))
        .or_else(|| request.headers()
            .get("cf-connecting-ip") // Cloudflare
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    
    let user_agent = extract_header(request.headers(), "user-agent");

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %uri.path(),
        client_ip = %client_ip,
        user_agent = %user_agent.unwrap_or_else(|| "unknown".to_string()),
        "incoming request"
    );

    let mut response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    // Add request_id to response headers
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("error")),
    );

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
