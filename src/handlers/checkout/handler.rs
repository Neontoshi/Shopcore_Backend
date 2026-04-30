use axum::{
    extract::{State, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dtos::CheckoutRequest;
use crate::services::CheckoutService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use validator::Validate;

pub async fn checkout(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CheckoutRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }
    
    let order_id = CheckoutService::checkout(
        state.get_db_pool(),
        &auth_user.user_id,
        &req.shipping_address_id,
        &req.payment_method,
    ).await?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order placed successfully",
        "order_id": order_id
    })))
}