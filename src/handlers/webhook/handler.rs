use axum::{extract::State, Json, Request};
use stripe::{Webhook, Event, EventType, PaymentIntent};
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::services::OrderService;

pub async fn stripe_webhook(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<serde_json::Value>, AppError> {
    // Extract the webhook signature from headers
    let signature = request
        .headers()
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Missing stripe signature"))?;
    
    // Get the request body
    let body = axum::body::to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::bad_request(format!("Invalid body: {}", e)))?;
    
    // Verify the webhook signature
    let event = Webhook::construct_event(
        &body,
        signature,
        &state.config.stripe_webhook_secret,
    ).map_err(|e| AppError::payment_error(format!("Invalid signature: {}", e)))?;
    
    // Handle the event
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            let payment_intent: PaymentIntent = serde_json::from_value(event.data.object)
                .map_err(|e| AppError::payment_error(format!("Invalid payment intent: {}", e)))?;
            
            // Extract order_id from metadata
            if let Some(metadata) = payment_intent.metadata {
                if let Some(order_id_str) = metadata.get("order_id") {
                    let order_id = uuid::Uuid::parse_str(order_id_str)
                        .map_err(|_| AppError::payment_error("Invalid order ID in metadata"))?;
                    
                    // Update order payment status
                    OrderService::update_payment_status(state.get_db_pool(), &order_id, "paid").await?;
                    
                    tracing::info!("Payment succeeded for order: {}", order_id);
                }
            }
        }
        EventType::PaymentIntentPaymentFailed => {
            tracing::warn!("Payment failed: {:?}", event);
        }
        _ => {
            tracing::debug!("Unhandled webhook event: {:?}", event.type_);
        }
    }
    
    Ok(Json(serde_json::json!({"received": true})))
}