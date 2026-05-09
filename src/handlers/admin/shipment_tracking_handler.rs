use axum::{
    extract::{Path, State},
    Extension,
    Json,
};
use uuid::Uuid;
use sqlx::FromRow;
use crate::app::state::AppState;
use crate::dtos::shipment_tracking_dto::*;
use crate::errors::AppError;
use crate::services::shipment_tracking_service::ShipmentTrackingService;
use crate::middleware::auth::AuthUser;

#[derive(FromRow)]
struct OrderDetails {
    user_id: Uuid,
    order_number: String,
    user_email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

// Admin: Add tracking to order with email notification
pub async fn add_tracking(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
    Json(req): Json<AddTrackingRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Only admins can add tracking information"));
    }

    // Get order details and user info for email
    let order_info = sqlx::query_as!(
        OrderDetails,
        r#"
        SELECT 
            o.user_id as "user_id",
            o.order_number as "order_number",
            u.email as "user_email",
            u.first_name as "first_name",
            u.last_name as "last_name"
        FROM orders o
        JOIN users u ON o.user_id = u.id
        WHERE o.id = $1
        "#,
        order_id
    )
    .fetch_optional(state.get_db_pool())
    .await
    .map_err(AppError::from)?;

    let order_info = order_info.ok_or_else(|| AppError::not_found("Order not found"))?;

    // Add tracking
    ShipmentTrackingService::add_tracking(
        state.get_db_pool(),
        &order_id,
        &req.tracking_number,
        &req.carrier,
        req.estimated_delivery,
        true,
    ).await?;

    // Build customer name for email personalization
    let customer_name = match (order_info.first_name, order_info.last_name) {
        (Some(first), Some(last)) => format!("{} {}", first, last),
        (Some(first), None) => first,
        _ => "Valued Customer".to_string(),
    };
    
    // Generate tracking URL
    let tracking_url = ShipmentTrackingService::generate_tracking_url(&req.carrier, &req.tracking_number);
    
    // Send personalized shipment notification email
    let email_service = state.get_email_service();
    
    // Use the enhanced shipment notification method
    if let Err(e) = email_service.send_shipment_notification(
        &order_info.user_email,
        &customer_name,
        &order_info.order_number,
        &req.tracking_number,
        &req.carrier,
        &tracking_url,
        req.estimated_delivery,
    ).await {
        tracing::error!("Failed to send shipment notification to {}: {}", order_info.user_email, e);
    } else {
        tracing::info!("Shipment notification sent to {} for order {}", customer_name, order_info.order_number);
    }

    // Audit log: Record which admin added tracking
    // Note: You need to create the admin_audit_logs table first
    let _ = sqlx::query!(
        r#"
        INSERT INTO admin_audit_logs (admin_id, action, order_id, created_at) 
        VALUES ($1, 'add_tracking', $2, NOW())
        "#,
        auth_user.user_id,
        order_id
    )
    .execute(state.get_db_pool())
    .await;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Tracking added and notification sent to {}", customer_name),
        "order_id": order_id.to_string(),
        "customer_email": order_info.user_email,
        "tracking_number": req.tracking_number,
        "carrier": req.carrier,
        "tracking_url": tracking_url
    })))
}

// Admin: Update estimated delivery date
pub async fn update_estimated_delivery(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
    Json(req): Json<UpdateEstimatedDeliveryRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Only admins can update estimated delivery"));
    }
    
    ShipmentTrackingService::update_estimated_delivery(
        state.get_db_pool(),
        &order_id,
        req.estimated_delivery,
    ).await?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Estimated delivery date updated",
        "order_id": order_id.to_string()
    })))
}

// Admin: Mark order as delivered
pub async fn mark_delivered(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("Only admins can mark orders as delivered"));
    }

    // Get customer info before updating
    let customer_info = sqlx::query!(
        r#"
        SELECT u.email, u.first_name, u.last_name, o.order_number
        FROM orders o
        JOIN users u ON o.user_id = u.id
        WHERE o.id = $1
        "#,
        order_id
    )
    .fetch_optional(state.get_db_pool())
    .await
    .map_err(AppError::from)?;

    ShipmentTrackingService::mark_delivered(state.get_db_pool(), &order_id).await?;

    // Audit log: Record which admin marked as delivered
    let _ = sqlx::query!(
        r#"
        INSERT INTO admin_audit_logs (admin_id, action, order_id, created_at) 
        VALUES ($1, 'mark_delivered', $2, NOW())
        "#,
        auth_user.user_id,
        order_id
    )
    .execute(state.get_db_pool())
    .await;

    // Send delivery confirmation email if customer info exists
    if let Some(customer) = customer_info {
        let email_service = state.get_email_service();
        let customer_name = match (customer.first_name, customer.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first,
            _ => "Customer".to_string(),
        };
        
        // Send delivery confirmation using the enhanced method
        let _ = email_service.send_delivery_confirmation(
            &customer.email,
            &customer_name,
            &customer.order_number,
        ).await;
        
        tracing::info!("Delivery confirmation sent to {} for order {}", customer_name, customer.order_number);
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order marked as delivered and customer notified",
        "order_id": order_id.to_string()
    })))

}
