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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    pub status_code: u16,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>, status: StatusCode, code: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetail {
                message: message.into(),
                code: Some(code.into()),
                details: None,
                status_code: status.as_u16(),
            },
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.error.details = Some(details);
        self
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.error.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(self)).into_response()
    }
}