use axum::{
    extract::{State, Path, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dtos::{CreateAddressRequest, AddressResponse};
use crate::services::AddressService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use validator::Validate;

pub async fn create_address(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateAddressRequest>,
) -> Result<Json<AddressResponse>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let address = AddressService::create_address(
        state.get_db_pool(),
        &auth_user.user_id,
        req,
    ).await?;
    Ok(Json(address))
}

pub async fn get_addresses(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<AddressResponse>>, AppError> {
    let addresses = AddressService::get_user_addresses(
        state.get_db_pool(),
        &auth_user.user_id,
    ).await?;
    Ok(Json(addresses))
}

pub async fn update_address(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(address_id): Path<Uuid>,
    Json(req): Json<CreateAddressRequest>,
) -> Result<Json<AddressResponse>, AppError> {
    let address = AddressService::update_address(
        state.get_db_pool(),
        &auth_user.user_id,
        &address_id,
        req,
    ).await?;
    Ok(Json(address))
}

pub async fn delete_address(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(address_id): Path<Uuid>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    AddressService::delete_address(
        state.get_db_pool(),
        &auth_user.user_id,
        &address_id,
    ).await?;
    Ok(Json(crate::utils::MessageResponse::new("Address deleted successfully")))
}