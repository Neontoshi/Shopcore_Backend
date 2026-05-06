use axum::{
    extract::{Path, State},
    Extension,
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::services::shipment_tracking_service::ShipmentTrackingService;
use crate::middleware::auth::AuthUser;

// Customer: Get tracking info for their order
pub async fn get_order_tracking(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let tracking_info = ShipmentTrackingService::get_tracking_info(
        state.get_db_pool(),
        &order_id,
        &auth_user.user_id,
        auth_user.role.can_access_admin(),
    ).await?;
    
    match tracking_info {
        Some(info) => Ok(Json(serde_json::json!({
            "success": true,
            "data": info
        }))),
        None => Err(AppError::not_found("No tracking information found for this order")),
    }
}