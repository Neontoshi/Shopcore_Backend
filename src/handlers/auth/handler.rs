use axum::{extract::State, Json};
use crate::app::state::AppState;
use crate::dtos::{
    RegisterRequest, 
    LoginRequest, 
    RefreshTokenRequest, 
    AuthResponse, 
    ForgotPasswordRequest, 
    ResetPasswordRequest, 
    ChangePasswordRequest};
use crate::services::AuthService;
use crate::errors::AppError;
use validator::Validate;
use crate::middleware::auth::AuthUser;

use axum::extract::Request; 

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate request
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }
    
    let jwt_service = crate::utils::JwtService::new(
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    );
    
    let response = AuthService::register(
        state.get_db_pool(),
        &jwt_service,
        req,
    ).await?;
    
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate request
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }
    
    let jwt_service = crate::utils::JwtService::new(
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    );
    
    let response = AuthService::login(
        state.get_db_pool(),
        &jwt_service,
        req,
    ).await?;
    
    Ok(Json(response))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<crate::dtos::TokenResponse>, AppError> {
    let jwt_service = crate::utils::JwtService::new(
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    );
    
    let new_access_token = AuthService::refresh_access_token(
        state.get_db_pool(),
        &jwt_service,
        &req.refresh_token,
    ).await?;
    
    Ok(Json(crate::dtos::TokenResponse {
        access_token: new_access_token,
        refresh_token: req.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 86400,
    }))
}

pub async fn logout(
    _state: State<AppState>,
    _request: Request,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    // In a real implementation, you'd blacklist the token in Redis
    // For now, just return success
    Ok(Json(crate::utils::MessageResponse::new("Logged out successfully")))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(req): Json<ForgotPasswordRequest>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    AuthService::forgot_password(
        state.get_db_pool(),
        state.get_email_service(),
        &req.email,
    ).await?;

    // Always return success to avoid email enumeration
    Ok(Json(crate::utils::MessageResponse::new(
        "If that email exists, a reset link has been sent"
    )))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    AuthService::reset_password(
        state.get_db_pool(),
        &req.token,
        &req.new_password,
    ).await?;

    Ok(Json(crate::utils::MessageResponse::new("Password reset successfully")))
}

pub async fn change_password(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    req.validate().map_err(|e| AppError::validation(e.to_string()))?;

    AuthService::change_password(
        state.get_db_pool(),
        &auth_user.user_id,
        &req.current_password,
        &req.new_password,
    ).await?;

    Ok(Json(crate::utils::MessageResponse::new("Password changed successfully")))
}