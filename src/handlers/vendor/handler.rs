use axum::{
    extract::{State, Path, Query, Extension},
    Json,
};
use uuid::Uuid;
use validator::Validate;
use crate::app::state::AppState;
use crate::dtos::vendor_dto::{ApplyForVendorRequest, ReviewVendorApplicationRequest, VendorApplicationResponse, VendorProfileResponse};
use crate::services::vendor_service::VendorService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::types::PaginationParams;

// Customer: Apply to become a vendor
pub async fn apply_for_vendor(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<ApplyForVendorRequest>,
) -> Result<Json<VendorApplicationResponse>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let application = VendorService::apply_for_vendor(
        state.get_db_pool(),
        &auth_user.user_id,
        req,
    ).await?;

    Ok(Json(application))
}

// Customer: Get my application status
pub async fn get_my_application(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Option<VendorApplicationResponse>>, AppError> {
    let application = VendorService::get_my_application(
        state.get_db_pool(),
        &auth_user.user_id,
    ).await?;

    Ok(Json(application))
}

// Customer: Get my vendor profile (if approved)
pub async fn get_my_vendor_profile(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Option<VendorProfileResponse>>, AppError> {
    let profile = VendorService::get_vendor_profile(
        state.get_db_pool(),
        &auth_user.user_id,
    ).await?;

    Ok(Json(profile))
}

// ADMIN: Get all pending applications
pub async fn get_pending_applications(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<VendorApplicationResponse>>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Admin access required"));
    }

    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).max(1);

    let (applications, _total) = VendorService::get_pending_applications(
        state.get_db_pool(),
        page,
        page_size,
    ).await?;

    Ok(Json(applications))
}

// ADMIN: Review an application
pub async fn review_application(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(application_id): Path<Uuid>,
    Json(req): Json<ReviewVendorApplicationRequest>,
) -> Result<Json<VendorApplicationResponse>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Admin access required"));
    }

    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let status = req.status.ok_or_else(|| AppError::bad_request("Status is required"))?;

    if status != "approved" && status != "rejected" {
        return Err(AppError::bad_request("Status must be 'approved' or 'rejected'"));
    }

    let application = VendorService::review_application(
        state.get_db_pool(),
        &application_id,
        &auth_user.user_id,
        &status,
        req.admin_notes,
    ).await?;

    Ok(Json(application))
}