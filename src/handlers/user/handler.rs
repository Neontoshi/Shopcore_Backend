use axum::{extract::{State, Path}, Json};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::services::UserService;
use crate::dtos::{UpdateProfileRequest, ProfileResponse};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;

pub async fn get_profile(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ProfileResponse>, AppError> {
    if auth_user.user_id != user_id && !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Access denied"));
    }

    let profile = UserService::get_profile(state.get_db_pool(), &user_id).await?;
    Ok(Json(profile))
}

pub async fn update_profile(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    if auth_user.user_id != user_id && !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Access denied"));
    }

    let profile = UserService::update_profile(state.get_db_pool(), &user_id, req).await?;
    Ok(Json(profile))
}