use axum::{
    extract::{State, Request},
    http::StatusCode,
    body::to_bytes,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::app::state::AppState;
use crate::errors::AppError;

pub async fn stripe_webhook(
    State(state): State<AppState>,
    request: Request,
) -> Result<StatusCode, AppError> {
    let signature = request
        .headers()
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Missing stripe-signature header"))?
        .to_string();

    let body_bytes = to_bytes(request.into_body(), usize::MAX)
        .await
        .map_err(|_| AppError::bad_request("Failed to read body"))?;

    verify_stripe_signature(&body_bytes, &signature, &state.config.stripe_webhook_secret)?;

    let event: serde_json::Value = serde_json::from_slice(&body_bytes)
        .map_err(|_| AppError::bad_request("Invalid JSON"))?;

    let event_type = event["type"].as_str().unwrap_or("");

    match event_type {
        "payment_intent.succeeded" => {
            let payment_intent_id = event["data"]["object"]["id"]
                .as_str()
                .unwrap_or("");
            let order_id = event["data"]["object"]["metadata"]["order_id"]
                .as_str()
                .unwrap_or("");

            if !order_id.is_empty() && !payment_intent_id.is_empty() {
                let order_uuid = order_id.parse::<uuid::Uuid>()
                    .map_err(|_| AppError::bad_request("Invalid order_id in metadata"))?;

                // Update payment transaction
                sqlx::query!(
                    "UPDATE payment_transactions SET status = 'completed', updated_at = NOW()
                     WHERE order_id = $1 AND provider_transaction_id = $2",
                    order_uuid,
                    payment_intent_id
                )
                .execute(state.get_db_pool())
                .await?;

                // Update order status
                sqlx::query!(
                    "UPDATE orders SET payment_status = 'paid', status = 'confirmed', updated_at = NOW()
                     WHERE id = $1",
                    order_uuid
                )
                .execute(state.get_db_pool())
                .await?;

                // Clear the user's cart
                let cart_cleared = sqlx::query!(
                    r#"
                    DELETE FROM cart_items 
                    WHERE cart_id IN (
                        SELECT c.id FROM carts c
                        JOIN orders o ON c.user_id = o.user_id
                        WHERE o.id = $1
                        AND c.expires_at > NOW()
                        ORDER BY c.created_at DESC
                        LIMIT 1
                    )
                    "#,
                    order_uuid
                )
                .execute(state.get_db_pool())
                .await;

                if let Ok(result) = cart_cleared {
                    tracing::info!("Cart cleared for order {}: {} items removed", order_uuid, result.rows_affected());
                } else {
                    tracing::warn!("Failed to clear cart for order {}", order_uuid);
                }
            }
        }

        "payment_intent.payment_failed" => {
            let payment_intent_id = event["data"]["object"]["id"]
                .as_str()
                .unwrap_or("");
            let failure_reason = event["data"]["object"]["last_payment_error"]["message"]
                .as_str()
                .unwrap_or("Payment failed");
            let order_id = event["data"]["object"]["metadata"]["order_id"]
                .as_str()
                .unwrap_or("");

            if !order_id.is_empty() {
                let order_uuid = order_id.parse::<uuid::Uuid>()
                    .map_err(|_| AppError::bad_request("Invalid order_id in metadata"))?;

                sqlx::query!(
                    "UPDATE payment_transactions SET status = 'failed', failure_reason = $3, updated_at = NOW()
                     WHERE order_id = $1 AND provider_transaction_id = $2",
                    order_uuid,
                    payment_intent_id,
                    failure_reason
                )
                .execute(state.get_db_pool())
                .await?;

                sqlx::query!(
                    "UPDATE orders SET payment_status = 'failed', updated_at = NOW() WHERE id = $1",
                    order_uuid
                )
                .execute(state.get_db_pool())
                .await?;

                let cancel_result = sqlx::query!(
                    "SELECT user_id FROM orders WHERE id = $1",
                    order_uuid
                )
                .fetch_optional(state.get_db_pool())
                .await;

                if let Ok(Some(_order)) = cancel_result {
                    // Restore stock from order items
                    let items = sqlx::query!(
                        "SELECT product_id, quantity FROM order_items WHERE order_id = $1",
                        order_uuid
                    )
                    .fetch_all(state.get_db_pool())
                    .await;

                    if let Ok(items) = items {
                        for item in items {
                            let _ = sqlx::query!(
                                "UPDATE products SET stock_quantity = stock_quantity + $1, updated_at = NOW() WHERE id = $2",
                                item.quantity,
                                item.product_id
                            )
                            .execute(state.get_db_pool())
                            .await;
                        }
                    }

                    let _ = sqlx::query!(
                        "UPDATE orders SET status = 'cancelled', updated_at = NOW() WHERE id = $1",
                        order_uuid
                    )
                    .execute(state.get_db_pool())
                    .await;
                }
            }
        }

        _ => {}
    }

    Ok(StatusCode::OK)
}

fn verify_stripe_signature(
    body: &[u8],
    signature_header: &str,
    secret: &str,
) -> Result<(), AppError> {
    let mut timestamp = "";
    let mut signatures: Vec<&str> = vec![];

    for part in signature_header.split(',') {
        if let Some(t) = part.strip_prefix("t=") {
            timestamp = t;
        } else if let Some(s) = part.strip_prefix("v1=") {
            signatures.push(s);
        }
    }

    if timestamp.is_empty() || signatures.is_empty() {
        return Err(AppError::bad_request("Invalid stripe-signature format"));
    }

    let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(body));

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| AppError::internal_server_error())?;
    mac.update(signed_payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());

    if signatures.iter().any(|s| *s == expected.as_str()) {
        Ok(())
    } else {
        Err(AppError::bad_request("Invalid Stripe signature"))
    }
}