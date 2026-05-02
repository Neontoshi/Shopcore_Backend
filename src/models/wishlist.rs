use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;  // Add this import

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Wishlist {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WishlistItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub slug: String,
    pub price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub image_url: Option<String>,
    pub average_rating: Decimal,
    pub total_reviews: i32,
    pub added_at: DateTime<Utc>,
}