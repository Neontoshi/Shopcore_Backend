use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PasswordReset {
    pub email: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}