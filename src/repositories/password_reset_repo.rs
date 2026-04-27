use sqlx::PgPool;
use crate::models::PasswordReset;
use crate::errors::AppError;
use chrono::{DateTime, Utc};

pub struct PasswordResetRepository;

impl PasswordResetRepository {
    pub async fn create(
        pool: &PgPool,
        email: &str,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        // Delete any existing tokens for this email first
        sqlx::query!(
            "DELETE FROM password_resets WHERE email = $1",
            email
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            "INSERT INTO password_resets (email, token, expires_at) VALUES ($1, $2, $3)",
            email,
            token,
            expires_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<PasswordReset>, AppError> {
        let reset = sqlx::query_as!(
            PasswordReset,
            "SELECT email, token, expires_at FROM password_resets WHERE token = $1",
            token
        )
        .fetch_optional(pool)
        .await?;

        Ok(reset)
    }

    pub async fn delete_by_email(
        pool: &PgPool,
        email: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM password_resets WHERE email = $1",
            email
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}