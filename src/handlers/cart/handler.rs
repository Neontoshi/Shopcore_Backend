use axum::{
    extract::{State, Path, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dtos::{AddToCartRequest, UpdateCartRequest, CartResponse};
use crate::services::CartService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use validator::Validate;

pub async fn get_cart(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<CartResponse>, AppError> {
    let cart = CartService::get_cart(state.get_db_pool(), &auth_user.user_id).await?;
    Ok(Json(cart.into()))
}

pub async fn add_to_cart(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<AddToCartRequest>,
) -> Result<Json<CartResponse>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let cart = CartService::add_to_cart(state.get_db_pool(), &auth_user.user_id, req).await?;
    Ok(Json(cart.into()))
}

pub async fn update_cart_item(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(item_id): Path<Uuid>,
    Json(req): Json<UpdateCartRequest>,
) -> Result<Json<CartResponse>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let cart = CartService::update_cart_item(
        state.get_db_pool(),
        &auth_user.user_id,
        &item_id,
        req,
    ).await?;
    Ok(Json(cart.into()))
}

pub async fn remove_from_cart(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<CartResponse>, AppError> {
    let cart = CartService::remove_from_cart(
        state.get_db_pool(),
        &auth_user.user_id,
        &item_id,
    ).await?;
    Ok(Json(cart.into()))
}

pub async fn clear_cart(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<crate::utils::MessageResponse>, AppError> {
    CartService::clear_cart(state.get_db_pool(), &auth_user.user_id).await?;
    Ok(Json(crate::utils::MessageResponse::new("Cart cleared successfully")))
}