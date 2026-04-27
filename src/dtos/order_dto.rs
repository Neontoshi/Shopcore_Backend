use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::constants::order_status::OrderStatus;

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub subtotal: String,
    pub tax: String,
    pub shipping_cost: String,
    pub total: String,
    pub payment_method: String,
    pub payment_status: String,
    pub created_at: DateTime<Utc>,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
pub struct OrderSummaryResponse {
    pub id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub total: String,
    pub created_at: DateTime<Utc>,
    pub item_count: i64,
}

#[derive(Debug, Serialize)]
pub struct CheckoutResponse {
    pub order_id: Uuid,
    pub order_number: String,
    pub total: String,
    pub payment_url: Option<String>,
}