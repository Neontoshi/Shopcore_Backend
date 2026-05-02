// src/handlers/webhook/coinbase.rs
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    body::Bytes,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use serde_json::Value;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

pub async fn coinbase_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let sig = headers
        .get("X-CC-Webhook-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Missing webhook signature"))?;

    let secret = std::env::var("COINBASE_COMMERCE_WEBHOOK_SECRET")
        .map_err(|_| AppError::bad_request("Missing webhook secret"))?;

    verify_signature(&body, sig, &secret)?;

    let event: Value = serde_json::from_slice(&body)
        .map_err(|_| AppError::bad_request("Invalid JSON"))?;

    let event_type = event["event"]["type"].as_str().unwrap_or("");
    let charge_id  = event["event"]["data"]["id"].as_str().unwrap_or("");
    let order_id_str = event["event"]["data"]["metadata"]["order_id"].as_str().unwrap_or("");

    if charge_id.is_empty() || order_id_str.is_empty() {
        return Ok(StatusCode::OK);
    }

    let (payment_status, order_status) = match event_type {
        "charge:confirmed" | "charge:resolved" => ("completed", Some("processing")),
        "charge:failed"                         => ("failed",    None),
        "charge:delayed"                        => ("pending",   None),
        "charge:pending"                        => ("pending",   None),
        _ => return Ok(StatusCode::OK),
    };

    let order_uuid = Uuid::parse_str(order_id_str)
        .map_err(|_| AppError::bad_request("Invalid order_id in webhook metadata"))?;

    let pool = state.get_db_pool();

    sqlx::query!(
        "UPDATE payment_transactions SET status = $1, updated_at = NOW() WHERE provider_transaction_id = $2",
        payment_status,
        charge_id,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    if let Some(new_order_status) = order_status {
        sqlx::query!(
            r#"
            UPDATE orders SET status = $1, updated_at = NOW()
            WHERE id = $2 AND status NOT IN ('cancelled', 'delivered', 'completed')
            "#,
            new_order_status,
            order_uuid,
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;
    }

    Ok(StatusCode::OK)
}

fn verify_signature(body: &[u8], signature: &str, secret: &str) -> Result<(), AppError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| AppError::bad_request("HMAC init failed"))?;
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());
    if expected != signature {
        return Err(AppError::bad_request("Invalid webhook signature"));
    }
    Ok(())
}