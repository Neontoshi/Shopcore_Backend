use reqwest::Client as HttpClient;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sqlx::PgPool;
use uuid::Uuid;
use crate::config::app_config::AppConfig;
use crate::errors::AppError;

pub struct PaymentService;

impl PaymentService {
    // STRIPE via direct HTTP
    pub async fn create_stripe_payment(
    amount: Decimal,
    order_id: Uuid,
    config: &AppConfig,
) -> Result<(String, String), AppError> {
    let client = HttpClient::new();
    let amount_cents = (amount * Decimal::new(100, 0))
        .to_u64()
        .ok_or_else(|| AppError::payment_error("Invalid amount"))?;

    // Stripe requires x-www-form-urlencoded, not JSON
    let params = [
        ("amount", amount_cents.to_string()),
        ("currency", "usd".to_string()),
        ("metadata[order_id]", order_id.to_string()),
        ("automatic_payment_methods[enabled]", "true".to_string()),
    ];

    let response = client
        .post("https://api.stripe.com/v1/payment_intents")
        .header("Authorization", format!("Bearer {}", config.stripe_secret_key))
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::payment_error(e.to_string()))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::payment_error(e.to_string()))?;

    let payment_id = result["id"]
        .as_str()
        .ok_or_else(|| AppError::payment_error("No payment ID"))?
        .to_string();

    let client_secret = result["client_secret"]
        .as_str()
        .ok_or_else(|| AppError::payment_error("No client secret"))?
        .to_string();

    Ok((payment_id, client_secret))
}
    // BANK TRANSFER
    pub fn create_bank_transfer_reference(order_id: Uuid, order_number: &str) -> String {
        format!("INV-{}-{}", order_number, order_id.simple())
    }
    
    // CRYPTO (ETH/Solana via Coinbase Commerce)
    pub async fn create_crypto_payment(
        amount: Decimal,
        order_id: Uuid,
        order_number: &str,
        crypto_type: &str,
    ) -> Result<(String, String), AppError> {
        let client = HttpClient::new();
        let api_key = std::env::var("COINBASE_COMMERCE_API_KEY")
            .map_err(|_| AppError::payment_error("Missing Coinbase API key"))?;
        
        let body = serde_json::json!({
            "name": format!("Order {}", order_number),
            "description": format!("Payment for order {}", order_number),
            "pricing_type": "fixed_price",
            "local_price": {
                "amount": amount.to_string(),
                "currency": "USD"
            },
            "metadata": {
                "order_id": order_id.to_string(),
                "order_number": order_number,
                "blockchain": crypto_type
            },
            "redirect_url": format!("{}/orders/{}", std::env::var("APP_URL").unwrap_or("http://localhost:5173".into()), order_id),
            "cancel_url": format!("{}/cart", std::env::var("APP_URL").unwrap_or("http://localhost:5173".into()))
        });
        
        let response = client.post("https://api.commerce.coinbase.com/charges")
            .header("X-CC-Api-Key", api_key)
            .header("X-CC-Version", "2018-03-22")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::payment_error(e.to_string()))?;
        
        let result: serde_json::Value = response.json().await
            .map_err(|e| AppError::payment_error(e.to_string()))?;
        
        let charge_id = result["data"]["id"].as_str()
            .ok_or_else(|| AppError::payment_error("No charge ID"))?
            .to_string();
        
        let hosted_url = result["data"]["hosted_url"].as_str()
            .ok_or_else(|| AppError::payment_error("No hosted URL"))?
            .to_string();
        
        Ok((charge_id, hosted_url))
    }
    
    // Record payment transaction
    pub async fn record_transaction(
        pool: &PgPool,
        order_id: &Uuid,
        amount: Decimal,
        payment_method: &str,
        provider_tx_id: &str,
        status: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO payment_transactions (order_id, amount, currency, status, payment_method, provider_transaction_id)
            VALUES ($1, $2, 'USD', $3, $4, $5)
            "#,
            order_id,
            amount,
            status,
            payment_method,
            provider_tx_id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
}