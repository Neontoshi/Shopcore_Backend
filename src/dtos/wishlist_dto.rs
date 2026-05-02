use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct AddToWishlistRequest {
    pub product_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct WishlistItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub slug: String,
    pub price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub image_url: Option<String>,
    pub average_rating: f64,
    pub total_reviews: i32,
    pub added_at: DateTime<Utc>,
}