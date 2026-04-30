use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CartItem {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price_at_add: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CartWithItems {
    pub cart: Cart,
    pub items: Vec<CartItemWithProduct>,
    pub total_items: i32,
    pub subtotal: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CartItemWithProduct {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub slug: String,
    pub quantity: i32,
    pub price: Decimal,
    pub total: Option<Decimal>,  // Postgres computed column returns Option
    pub image_url: Option<String>,
}