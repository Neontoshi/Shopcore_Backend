use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub shipping_address_id: Uuid,
    pub billing_address_id: Uuid,
    pub payment_method: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckoutResponse {
    pub order_id: Uuid,
    pub order_number: String,
    pub total: String,
    pub payment_url: Option<String>,
    pub message: String,  // Add this field
}