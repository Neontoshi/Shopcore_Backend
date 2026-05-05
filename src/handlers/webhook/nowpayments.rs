use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    body::Bytes,
};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use hex;
use serde_json::Value;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::errors::AppError;

type HmacSha512 = Hmac<Sha512>;

pub async fn nowpayments_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, AppError> {
    let sig = headers
        .get("x-nowpayments-sig")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Missing webhook signature"))?;

    let secret = std::env::var("NOWPAYMENTS_IPN_SECRET")
        .map_err(|_| AppError::bad_request("Missing IPN secret"))?;

    verify_signature(&body, sig, &secret)?;

    let event: Value = serde_json::from_slice(&body)
        .map_err(|_| AppError::bad_request("Invalid JSON"))?;

    let payment_status = event["payment_status"].as_str().unwrap_or("");
    let invoice_id     = event["invoice_id"].as_str().unwrap_or("");
    let order_id_str   = event["order_id"].as_str().unwrap_or("");

    if invoice_id.is_empty() || order_id_str.is_empty() {
        return Ok(StatusCode::OK);
    }

    let (payment_status_db, order_status) = match payment_status {
        "finished"        => ("completed", Some("processing")),
        "failed"
        | "expired"       => ("failed", None),
        "partially_paid"  => ("partially_paid", None),
        _                 => return Ok(StatusCode::OK),
    };

    let order_uuid = Uuid::parse_str(order_id_str)
        .map_err(|_| AppError::bad_request("Invalid order_id in webhook"))?;

    let pool = state.get_db_pool();

    sqlx::query!(
        "UPDATE payment_transactions SET status = $1, updated_at = NOW()
         WHERE provider_transaction_id = $2",
        payment_status_db,
        invoice_id,
    )
    .execute(pool)
    .await
    .map_err(AppError::from)?;

    if let Some(new_order_status) = order_status {
        sqlx::query!(
            r#"UPDATE orders SET status = $1, updated_at = NOW()
               WHERE id = $2 AND status NOT IN ('cancelled', 'delivered', 'completed')"#,
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
    // NOWPayments signs the body with keys sorted alphabetically
    let sorted = sort_json_keys(body)?;
    let mut mac = HmacSha512::new_from_slice(secret.as_bytes())
        .map_err(|_| AppError::bad_request("HMAC init failed"))?;
    mac.update(sorted.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());
    if expected != signature.to_lowercase() {
        return Err(AppError::bad_request("Invalid webhook signature"));
    }
    Ok(())
}

fn sort_json_keys(body: &[u8]) -> Result<String, AppError> {
    let value: Value = serde_json::from_slice(body)
        .map_err(|_| AppError::bad_request("Invalid JSON for signature"))?;
    sorted_value(value)
        .map_err(|_| AppError::bad_request("Failed to re-serialize webhook body"))
}

fn sorted_value(value: Value) -> Result<String, serde_json::Error> {
    let v = sort_keys(value);
    serde_json::to_string(&v)
}

fn sort_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            let sorted = keys.into_iter()
                .map(|k| { let v = sort_keys(map[&k].clone()); (k, v) })
                .collect();
            Value::Object(sorted)
        }
        other => other,
    }
}