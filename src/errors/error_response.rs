// src/errors/error_response.rs
use serde::{Deserialize, Serialize};
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    pub code: Option<String>,
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetail {
                message: message.into(),
                code: None,
                details: None,
                status_code: None,
            },
        }
    }
    
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.error.code = Some(code.into());
        self
    }
    
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.error.details = Some(details);
        self
    }
    
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.error.status_code = Some(status.as_u16());
        self
    }
    
    // ADD THIS METHOD
    pub fn rate_limit() -> Self {
        Self::new("Too many requests. Please slow down and try again later.")
            .with_status(StatusCode::TOO_MANY_REQUESTS)
            .with_code("RATE_LIMIT_EXCEEDED")
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = self.error.status_code
            .map(StatusCode::from_u16)
            .and_then(Result::ok)
            .unwrap_or(StatusCode::BAD_REQUEST);
        
        (status, axum::Json(self)).into_response()
    }
}