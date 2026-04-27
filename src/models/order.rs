use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::constants::order_status::OrderStatus;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub subtotal: Decimal,
    pub tax: Decimal,
    pub shipping_cost: Decimal,
    pub total: Decimal,
    pub shipping_address_id: Uuid,
    pub billing_address_id: Uuid,
    pub payment_method: String,
    pub payment_status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: Decimal,
    pub total: Decimal,
    pub product_name: String,
    pub product_sku: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItems {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
    pub shipping_address: Address,
    pub billing_address: Address,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Address {
    pub id: Uuid,
    pub user_id: Uuid,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub is_default: bool,
    pub address_type: String, // 'shipping' or 'billing'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}