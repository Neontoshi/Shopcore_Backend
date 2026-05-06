use axum::{
    extract::State,
    Json,
};
use crate::app::state::AppState;
use crate::errors::AppError;

pub async fn carrier_webhook(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let carrier = payload.get("carrier").and_then(|v| v.as_str()).unwrap_or("");
    let tracking_number = payload.get("tracking_number").and_then(|v| v.as_str()).unwrap_or("");
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("");
    
    tracing::info!("Received carrier webhook: carrier={}, tracking={}, status={}", carrier, tracking_number, status);
    
    // Update order if delivered
    if status == "delivered" {
        let order = sqlx::query!(
            "SELECT id FROM orders WHERE tracking_number = $1",
            tracking_number
        )
        .fetch_optional(state.get_db_pool())
        .await
        .map_err(AppError::from)?;
        
        if let Some(order) = order {
            sqlx::query!(
                r#"
                UPDATE orders 
                SET delivered_at = NOW(), 
                    status = 'delivered', 
                    updated_at = NOW() 
                WHERE id = $1
                "#,
                order.id
            )
            .execute(state.get_db_pool())
            .await
            .map_err(AppError::from)?;
            
            tracing::info!("Order {} marked as delivered via webhook", order.id);
        }
    }
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Webhook processed"
    })))
}
