use axum::{
    middleware::Next,
    response::Response,
};

use axum::extract::Request;
use std::time::Instant;

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    tracing::info!(
        "{} {} - {} - {:?}",
        method,
        uri.path(),
        status,
        duration
    );
    
    response
}