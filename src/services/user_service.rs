use crate::models::{User};
use crate::repositories::UserRepository;
use crate::dtos::{UpdateProfileRequest, ChangePasswordRequest, UserResponse, ProfileResponse};
use crate::utils::PasswordService;
use crate::errors::AppError;
use crate::constants::roles::UserRole;
use sqlx::PgPool;
use uuid::Uuid;
use serde::Serialize;
use validator::Validate;

pub struct UserService;

impl UserService {
    pub async fn get_profile(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<ProfileResponse, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;
        
        // Get default address
        let default_address = UserRepository::get_default_address(pool, user_id)
            .await?
            .map(|addr| addr.into());
        
        Ok(ProfileResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone_number, // Map phone_number to phone for frontend
            role: user.role.to_str().to_string(),
            created_at: user.created_at,
            default_address,
        })
    }

    pub async fn update_profile(
        pool: &PgPool,
        user_id: &Uuid,
        req: UpdateProfileRequest,
    ) -> Result<ProfileResponse, AppError> {
        if let Err(errors) = req.validate() {
            return Err(AppError::validation(errors.to_string()));
        }

        // Update basic profile fields (map phone to phone_number for DB)
        UserRepository::update_profile(
            pool, 
            user_id, 
            req.first_name, 
            req.last_name,
            req.phone, // Frontend sends 'phone', DB expects 'phone_number'
        ).await?;

        // Update default address if provided
        if let Some(address) = req.default_address {
            UserRepository::upsert_default_address(pool, user_id, &address).await?;
        }

        // Return updated profile
        Self::get_profile(pool, user_id).await
    }

    pub async fn change_password(
        pool: &PgPool,
        user_id: &Uuid,
        req: ChangePasswordRequest,
    ) -> Result<(), AppError> {
        PasswordService::validate_password_strength(&req.new_password)
            .map_err(AppError::validation)?;

        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        let is_valid = PasswordService::verify(&req.current_password, &user.password_hash)
            .map_err(|_| AppError::internal_server_error())?;

        if !is_valid {
            return Err(AppError::validation("Current password is incorrect"));
        }

        let new_password_hash = PasswordService::hash(&req.new_password, 12)
            .map_err(|_| AppError::internal_server_error())?;

        UserRepository::update_password(pool, user_id, &new_password_hash).await?;
        Ok(())
    }

    pub async fn deactivate_account(pool: &PgPool, user_id: &Uuid) -> Result<(), AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        if !user.is_active {
            return Err(AppError::bad_request("Account is already deactivated"));
        }

        sqlx::query!(
            "UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1",
            user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;
        Ok(user.into())
    }

    pub async fn update_user_role(
        pool: &PgPool,
        user_id: &Uuid,
        new_role: UserRole,
    ) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        if user.role == new_role {
            return Err(AppError::bad_request("User already has this role"));
        }

        sqlx::query!(
            "UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2",
            new_role.to_str(),
            user_id
        )
        .execute(pool)
        .await?;

        let updated_user = UserRepository::find_by_id(pool, user_id)
            .await?
            .unwrap();
        Ok(updated_user.into())
    }

    pub async fn toggle_user_status(
        pool: &PgPool,
        user_id: &Uuid,
        is_active: bool,
    ) -> Result<UserResponse, AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        if user.is_active == is_active {
            let status = if is_active { "active" } else { "deactivated" };
            return Err(AppError::bad_request(format!("User is already {}", status)));
        }

        sqlx::query!(
            "UPDATE users SET is_active = $1, updated_at = NOW() WHERE id = $2",
            is_active,
            user_id
        )
        .execute(pool)
        .await?;

        let updated_user = UserRepository::find_by_id(pool, user_id)
            .await?
            .unwrap();
        Ok(updated_user.into())
    }

    pub async fn initiate_password_reset(pool: &PgPool, email: &str) -> Result<String, AppError> {
        UserRepository::find_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        let reset_token = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO password_resets (email, token, expires_at)
            VALUES ($1, $2, NOW() + INTERVAL '1 hour')
            ON CONFLICT (email) DO UPDATE
            SET token = $2, expires_at = NOW() + INTERVAL '1 hour'
            "#,
            email,
            reset_token
        )
        .execute(pool)
        .await?;

        Ok(reset_token)
    }

    pub async fn reset_password(
        pool: &PgPool,
        token: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        PasswordService::validate_password_strength(new_password)
            .map_err(AppError::validation)?;

        let reset_entry = sqlx::query!(
            "SELECT email FROM password_resets WHERE token = $1 AND expires_at > NOW()",
            token
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::bad_request("Invalid or expired reset token"))?;

        let password_hash = PasswordService::hash(new_password, 12)
            .map_err(|_| AppError::internal_server_error())?;

        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE email = $2",
            password_hash,
            reset_entry.email
        )
        .execute(pool)
        .await?;

        sqlx::query!("DELETE FROM password_resets WHERE token = $1", token)
            .execute(pool)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct UserStatistics {
    pub total_orders: i64,
    pub total_spent: String,
    pub pending_orders: i64,
    pub completed_orders: i64,
    pub total_reviews: i64,
    pub average_rating: f64,
    pub cart_items: i32,
    pub cart_total: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role.to_str().to_string(),
            is_active: user.is_active,
        }
    }
}

impl From<User> for ProfileResponse {
    fn from(user: User) -> Self {
        ProfileResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone_number, // Map DB phone_number to frontend phone
            role: user.role.to_str().to_string(),
            created_at: user.created_at,
            default_address: None, // Will be populated by get_profile method
        }
    }
}