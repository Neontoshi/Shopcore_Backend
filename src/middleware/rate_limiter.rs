use axum::{
    extract::State,
    middleware::Next,
    response::Response,
    Request,
    http::StatusCode,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration: Duration::from_secs(window_seconds),
        }
    }
    
    pub async fn check_rate_limit(&self, key: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        // Clean up old entries
        if let Some(timestamps) = requests.get_mut(key) {
            timestamps.retain(|&t| now.duration_since(t) < self.window_duration);
            
            if timestamps.len() >= self.max_requests as usize {
                return false;
            }
            
            timestamps.push(now);
        } else {
            requests.insert(key.to_string(), vec![now]);
        }
        
        true
    }
}

pub async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get client IP
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            request
                .extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|addr| addr.ip().to_string())
                .as_deref()
        })
        .unwrap_or("unknown");
    
    if !limiter.check_rate_limit(client_ip).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}