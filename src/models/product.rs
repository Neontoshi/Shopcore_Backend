use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub stock_quantity: i32,
    pub category_id: Option<Uuid>,
    pub sku: Option<String>,
    pub is_active: bool,
    pub image_url: Option<String>,
    pub weight: Option<Decimal>,
    pub average_rating: Decimal,
    pub total_reviews: i32,
    pub vendor_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Product {
    pub fn is_in_stock(&self) -> bool {
        self.stock_quantity > 0
    }
    
    pub fn has_discount(&self) -> bool {
        match self.compare_at_price {
            Some(compare_price) => compare_price > self.price,
            None => false,
        }
    }
    
    pub fn discount_percentage(&self) -> Option<f64> {
        if let Some(compare_price) = self.compare_at_price {
            if compare_price > self.price {
                let discount = ((compare_price - self.price) / compare_price) * Decimal::new(100, 0);
                Some(discount.to_f64().unwrap_or(0.0))
            } else {
                None
            }
        } else {
            None
        }
    }
}
