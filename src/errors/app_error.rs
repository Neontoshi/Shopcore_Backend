use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Error sending email: {0}")]
    EmailError(String),
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
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred while processing your request. \
                     The issue has been logged and our team will investigate. \
                     Please try again later or contact support if the problem persists."
                        .to_string(),
                )
            }
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                format!(
                    "The submitted data failed validation: {}. \
                     Please review the requirements and correct your input before resubmitting.",
                    msg
                ),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                format!(
                    "{} If you followed a link or typed the address manually, \
                     double-check for typos. The resource may have been moved, \
                     deleted, or never existed.",
                    msg
                ),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                format!(
                    "Authentication required: {}. \
                     Please log in with valid credentials. \
                     If your session has expired, obtain a new token and retry.",
                    msg
                ),
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                format!(
                    "Access denied: {}. \
                     Your account does not have the necessary permissions for this action. \
                     Contact your administrator if you believe this is a mistake.",
                    msg
                ),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                format!(
                    "The request could not be understood or was missing required parameters: {}. \
                     Please check the API documentation and ensure your request \
                     is correctly structured.",
                    msg
                ),
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                format!(
                    "The request conflicts with the current state of the resource: {}. \
                     This may be due to a duplicate entry or a concurrent modification. \
                     Resolve the conflict and try again.",
                    msg
                ),
            ),
            AppError::InternalServerError => {
                tracing::error!("Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred on our end while handling your request. \
                     The issue has been logged. Please try again in a few moments, \
                     or contact support if it continues."
                        .to_string(),
                )
            }
            AppError::EmailError(ref msg) => {
                tracing::error!("Email error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "We were unable to send the email at this time. \
                     This may be due to a temporary issue with our mail provider. \
                     Please verify your email address is correct and try again shortly."
                        .to_string(),
                )
            }
        };

        let body = serde_json::json!({
            "error": {
                "message": error_message,
                "status_code": status.as_u16(),
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

impl AppError {
    pub fn not_found(resource: &str) -> Self {
        AppError::NotFound(format!("'{}' was not found.", resource))
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
}