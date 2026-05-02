use axum::{extract::{State, Extension, Path}, Json};
use std::str::FromStr;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::services::PaymentService;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InitiatePaymentRequest {
    pub order_id: Uuid,
    pub payment_method: String, // "credit_card", "bank_transfer", "eth", "solana"
}

#[derive(Serialize)]
pub struct InitiatePaymentResponse {
    pub client_secret: Option<String>,
    pub checkout_url: Option<String>,
    pub charge_id: Option<String>,
    pub reference: Option<String>,
    pub message: String,
}

pub async fn initiate_payment(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<InitiatePaymentRequest>,
) -> Result<Json<InitiatePaymentResponse>, AppError> {
    let order = sqlx::query!(
        "SELECT order_number, total, user_id FROM orders WHERE id = $1",
        req.order_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Order not found"))?;

    if order.user_id != auth_user.user_id {
        return Err(AppError::forbidden("Not your order"));
    }

    let total = order.total;
    let total: rust_decimal::Decimal = rust_decimal::Decimal::from_str(&total.to_string())
        .unwrap_or(rust_decimal::Decimal::ZERO);

    match req.payment_method.as_str() {
        "credit_card" => {
            let (provider_tx_id, client_secret) = PaymentService::create_stripe_payment(
                total,
                req.order_id,
                &state.config,
            ).await?;

            PaymentService::record_transaction(
                state.get_db_pool(),
                &req.order_id,
                total,
                "credit_card",
                &provider_tx_id,
                "pending",
            ).await?;

            Ok(Json(InitiatePaymentResponse {
                client_secret: Some(client_secret),
                checkout_url: None,
                charge_id: None,
                reference: None,
                message: "Payment intent created".into(),
            }))
        }

        "bank_transfer" => {
            let reference = PaymentService::create_bank_transfer_reference(
                req.order_id,
                &order.order_number,
            );

            PaymentService::record_transaction(
                state.get_db_pool(),
                &req.order_id,
                total,
                "bank_transfer",
                &reference,
                "pending",
            ).await?;

            Ok(Json(InitiatePaymentResponse {
                client_secret: None,
                checkout_url: None,
                charge_id: None,
                reference: Some(reference),
                message: "Bank transfer instructions sent".into(),
            }))
        }

        "eth" | "solana" => {
            let (charge_id, checkout_url) = PaymentService::create_crypto_payment(
                total,
                req.order_id,
                &order.order_number,
                req.payment_method.as_str(),
            ).await?;

            PaymentService::record_transaction(
                state.get_db_pool(),
                &req.order_id,
                total,
                &format!("crypto_{}", req.payment_method),
                &charge_id,
                "pending",
            ).await?;

            Ok(Json(InitiatePaymentResponse {
                client_secret: None,
                checkout_url: Some(checkout_url),
                charge_id: Some(charge_id),
                reference: None,
                message: "Crypto payment initiated".into(),
            }))
        }

        _ => Err(AppError::bad_request("Invalid payment method")),
    }
}

pub async fn get_crypto_status(
    State(_state): State<AppState>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(charge_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let api_key = std::env::var("COINBASE_COMMERCE_API_KEY")
        .map_err(|_| AppError::payment_error("Missing Coinbase API key"))?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.commerce.coinbase.com/charges/{}", charge_id))
        .header("X-CC-Api-Key", api_key)
        .header("X-CC-Version", "2018-03-22")
        .send()
        .await
        .map_err(|e| AppError::payment_error(e.to_string()))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::payment_error(e.to_string()))?;

    let status = result["data"]["timeline"]
        .as_array()
        .and_then(|t| t.last())
        .and_then(|e| e["status"].as_str())
        .unwrap_or("pending")
        .to_lowercase();

    Ok(Json(serde_json::json!({ "status": status, "charge_id": charge_id })))
}