use crate::models::User;
use crate::repositories::UserRepository;
use crate::utils::{JwtService, PasswordService};
use crate::constants::roles::UserRole;
use crate::dtos::{RegisterRequest, LoginRequest, AuthResponse, UserResponse};
use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use crate::repositories::PasswordResetRepository;
use crate::services::EmailService;


pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        jwt_service: &JwtService,
        req: RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        // Check if user already exists
        let existing_user = UserRepository::find_by_email(pool, &req.email).await?;
        if existing_user.is_some() {
            return Err(AppError::conflict("User with this email already exists"));
        }
        
        // Hash password
        let password_hash = PasswordService::hash(&req.password, 12)
            .map_err(|_| AppError::internal_server_error())?;
        
        // Create user
        let user = User {
            id: Uuid::new_v4(),
            email: req.email.clone(),
            password_hash,
            first_name: req.first_name,
            last_name: req.last_name,
            phone_number: req.phone,
            role: UserRole::Customer,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_login_at: None,
        };
        
        UserRepository::create(pool, &user).await?;
        
        // Generate tokens
        let access_token = jwt_service.generate_access_token(
            &user.id,
            &user.email,
            user.role.to_str(),
        ).map_err(|_| AppError::internal_server_error())?;
        
        let refresh_token = jwt_service.generate_refresh_token(&user.id)
            .map_err(|_| AppError::internal_server_error())?;
        
        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 86400, // 24 hours in seconds
            user: UserResponse {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                role: user.role.to_str().to_string(),
                is_active: user.is_active,
            },
        })
    }
    
    pub async fn login(
        pool: &PgPool,
        jwt_service: &JwtService,
        req: LoginRequest,
    ) -> Result<AuthResponse, AppError> {
        // Find user by email
        let user = UserRepository::find_by_email(pool, &req.email)
            .await?
            .ok_or_else(|| AppError::unauthorized("Invalid email or password"))?;
        
        // Check if user is active
        if !user.is_active {
            return Err(AppError::unauthorized("Account is deactivated"));
        }
        
        // Verify password
        let is_valid = PasswordService::verify(&req.password, &user.password_hash)
            .map_err(|_| AppError::internal_server_error())?;
        
        if !is_valid {
            return Err(AppError::unauthorized("Invalid email or password"));
        }
        
        // Update last login
        UserRepository::update_last_login(pool, &user.id).await?;
        
        // Generate tokens
        let access_token = jwt_service.generate_access_token(
            &user.id,
            &user.email,
            user.role.to_str(),
        ).map_err(|_| AppError::internal_server_error())?;
        
        let refresh_token = jwt_service.generate_refresh_token(&user.id)
            .map_err(|_| AppError::internal_server_error())?;
        
        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 86400,
            user: UserResponse {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                role: user.role.to_str().to_string(),
                is_active: user.is_active,
            },
        })
    }
    
    pub async fn refresh_access_token(
        pool: &PgPool,
        jwt_service: &JwtService,
        refresh_token: &str,
    ) -> Result<String, AppError> {
        // Verify refresh token
        let token_data = jwt_service.verify_refresh_token(refresh_token)
            .map_err(|_| AppError::unauthorized("Invalid refresh token"))?;
        
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::unauthorized("Invalid user ID in token"))?;
        
        // Get user
        let user = UserRepository::find_by_id(pool, &user_id)
            .await?
            .ok_or_else(|| AppError::unauthorized("User not found"))?;
        
        // Generate new access token
        let new_access_token = jwt_service.generate_access_token(
            &user.id,
            &user.email,
            user.role.to_str(),
        ).map_err(|_| AppError::internal_server_error())?;
        
        Ok(new_access_token)
    }
    
// inside impl AuthService, add these four:

    pub async fn forgot_password(
        pool: &PgPool,
        email_service: &EmailService,
        email: &str,
    ) -> Result<(), AppError> {
        // Always return Ok to avoid email enumeration
        let user = UserRepository::find_by_email(pool, email).await?;
        if user.is_none() {
            return Ok(());
        }

        // Generate a secure random token
        let token = uuid::Uuid::new_v4().simple().to_string()
            + &uuid::Uuid::new_v4().simple().to_string();

        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

        PasswordResetRepository::create(pool, email, &token, expires_at).await?;

        email_service.send_password_reset_email(email, &token).await?;

        Ok(())
    }

    pub async fn reset_password(
        pool: &PgPool,
        token: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // Find the reset record
        let reset = PasswordResetRepository::find_by_token(pool, token)
            .await?
            .ok_or_else(|| AppError::bad_request("Invalid or expired reset token"))?;

        // Check expiry
        if reset.expires_at < chrono::Utc::now() {
            PasswordResetRepository::delete_by_email(pool, &reset.email).await?;
            return Err(AppError::bad_request("Reset token has expired"));
        }

        // Find user
        let user = UserRepository::find_by_email(pool, &reset.email)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        // Hash new password
        let password_hash = PasswordService::hash(new_password, 12)
            .map_err(|_| AppError::internal_server_error())?;

        // Update password
        UserRepository::update_password(pool, &user.id, &password_hash).await?;

        // Delete used token
        PasswordResetRepository::delete_by_email(pool, &reset.email).await?;

        Ok(())
    }

    pub async fn logout(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<(), AppError> {
        // Update last_login to signal session end — 
        // for full token blacklisting you'd need a redis store
        UserRepository::update_last_login(pool, user_id).await?;
        Ok(())
    }

    pub async fn change_password(
        pool: &PgPool,
        user_id: &Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User"))?;

        // Verify current password
        let is_valid = PasswordService::verify(current_password, &user.password_hash)
            .map_err(|_| AppError::internal_server_error())?;

        if !is_valid {
            return Err(AppError::unauthorized("Current password is incorrect"));
        }

        // Hash and update new password
        let password_hash = PasswordService::hash(new_password, 12)
            .map_err(|_| AppError::internal_server_error())?;

        UserRepository::update_password(pool, user_id, &password_hash).await?;

        Ok(())
    }
}