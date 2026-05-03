use axum::{
    middleware::Next,
    response::Response,
    extract::Request,
    body::Body,
};
use axum::http::{header, Method};
use serde_json::Value;

pub async fn sanitize_input_middleware(
    request: Request,
    next: Next,
) -> Response {
    // Only sanitize POST, PUT, PATCH requests
    if matches!(*request.method(), Method::POST | Method::PUT | Method::PATCH) {
        if let Some(content_type) = request.headers().get(header::CONTENT_TYPE) {
            if content_type.to_str().unwrap_or("").contains("application/json") {
                let (parts, body) = request.into_parts();
                let bytes = match axum::body::to_bytes(body, usize::MAX).await {
                    Ok(b) => b,
                    Err(_) => return next.run(Request::from_parts(parts, Body::empty())).await,
                };
                
                // Try to sanitize JSON body
                if let Ok(mut json) = serde_json::from_slice::<Value>(&bytes) {
                    sanitize_json_values(&mut json);
                    if let Ok(sanitized_bytes) = serde_json::to_vec(&json) {
                        let new_body = Body::from(sanitized_bytes);
                        let new_request = Request::from_parts(parts, new_body);
                        return next.run(new_request).await;
                    }
                }
                
                let new_request = Request::from_parts(parts, Body::from(bytes));
                return next.run(new_request).await;
            }
        }
    }
    
    next.run(request).await
}

fn sanitize_json_values(value: &mut Value) {
    match value {
        Value::String(s) => {
            // Basic sanitization of string values
            *s = s
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('&', "&amp;");
        }
        Value::Array(arr) => {
            for item in arr {
                sanitize_json_values(item);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                sanitize_json_values(v);
            }
        }
        _ => {}
    }
}
