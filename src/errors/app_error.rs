use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use serde_json::json;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Conflict(String),
    #[error("Internal server error")]
    InternalServerError,
    #[error("{0}")]
    EmailError(String),
    #[error("{0}")]
    PaymentError(String),
    #[error("Too many requests")]
    RateLimit,  // ADD THIS
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        match e.downcast::<sqlx::Error>() {
            Ok(sqlx_err) => AppError::Database(sqlx_err),
            Err(e) => {
                tracing::error!("Internal error: {}", e);
                AppError::InternalServerError
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again later.".to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong. Please try again.".to_string()),
            AppError::EmailError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::PaymentError(msg) => (StatusCode::PAYMENT_REQUIRED, msg),
            AppError::RateLimit => (StatusCode::TOO_MANY_REQUESTS, "Too many requests. Please slow down and try again later.".to_string()),  // ADD THIS
        };

        let body = json!({
            "error": {
                "message": message,
                "status_code": status.as_u16(),
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

impl AppError {
    pub fn not_found(resource: &str) -> Self {
        AppError::NotFound(format!("{} not found", resource))
    }
    pub fn validation(message: impl Into<String>) -> Self {
        AppError::Validation(message.into())
    }
    pub fn bad_request(message: impl Into<String>) -> Self {
        AppError::BadRequest(message.into())
    }
    pub fn conflict(message: impl Into<String>) -> Self {
        AppError::Conflict(message.into())
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        AppError::Unauthorized(message.into())
    }
    pub fn forbidden(message: impl Into<String>) -> Self {
        AppError::Forbidden(message.into())
    }
    pub fn email_error(message: impl Into<String>) -> Self {
        AppError::EmailError(message.into())
    }
    pub fn internal_server_error() -> Self {
        AppError::InternalServerError
    }
    pub fn payment_error(message: impl Into<String>) -> Self {
        AppError::PaymentError(message.into())
    }
    pub fn rate_limit() -> Self {  // ADD THIS METHOD
        AppError::RateLimit
    }
}