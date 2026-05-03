use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use crate::errors::ErrorResponse;

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
    RateLimit,
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

impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        tracing::error!("Redis error: {}", e);
        AppError::InternalServerError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let response = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                ErrorResponse::new(
                    "Database error. Please try again later.",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                )
            }
            AppError::Validation(msg) => {
                ErrorResponse::new(msg, StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
            }
            AppError::NotFound(msg) => {
                ErrorResponse::new(msg, StatusCode::NOT_FOUND, "NOT_FOUND")
            }
            AppError::Unauthorized(msg) => {
                ErrorResponse::new(msg, StatusCode::UNAUTHORIZED, "UNAUTHORIZED")
            }
            AppError::Forbidden(msg) => {
                ErrorResponse::new(msg, StatusCode::FORBIDDEN, "FORBIDDEN")
            }
            AppError::BadRequest(msg) => {
                ErrorResponse::new(msg, StatusCode::BAD_REQUEST, "BAD_REQUEST")
            }
            AppError::Conflict(msg) => {
                ErrorResponse::new(msg, StatusCode::CONFLICT, "CONFLICT")
            }
            AppError::InternalServerError => {
                ErrorResponse::new(
                    "Something went wrong. Please try again.",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                )
            }
            AppError::EmailError(msg) => {
                tracing::error!("Email error: {}", msg);
                ErrorResponse::new(msg, StatusCode::INTERNAL_SERVER_ERROR, "EMAIL_ERROR")
            }
            AppError::PaymentError(msg) => {
                ErrorResponse::new(msg, StatusCode::PAYMENT_REQUIRED, "PAYMENT_ERROR")
            }
            AppError::RateLimit => {
                ErrorResponse::new(
                    "Too many requests. Please slow down and try again later.",
                    StatusCode::TOO_MANY_REQUESTS,
                    "RATE_LIMIT_EXCEEDED",
                )
            }
        };

        response.into_response()
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
    pub fn rate_limit() -> Self {
        AppError::RateLimit
    }
}