use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::{DateTime, Utc};

fn validate_price(price: &Decimal) -> Result<(), ValidationError> {
    if *price < Decimal::ZERO || *price > Decimal::from(999999) {
        return Err(ValidationError::new("invalid_price"));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub slug: String,
    pub description: Option<String>,
    #[validate(custom(function = "validate_price"))]
    pub price: Decimal,
    pub compare_at_price: Option<Decimal>,
    #[validate(range(min = 0))]
    pub stock_quantity: i32,
    pub category_id: Option<Uuid>,
    pub sku: Option<String>,
    pub image_url: Option<String>,
    pub weight: Option<Decimal>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub stock_quantity: Option<i32>,
    pub category_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub image_url: Option<String>,
    pub weight: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
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
    pub average_rating: Option<Decimal>,
    pub total_reviews: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub weight: Option<Decimal>,
    pub category: Option<CategoryInfo>,
}

#[derive(Debug, Serialize)]
pub struct CategoryInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductFilter {
    pub query: Option<String>,
    pub category_id: Option<Uuid>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub is_active: Option<bool>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub display_order: i32,
    pub weight: Option<Decimal>,
}

impl From<crate::models::Category> for CategoryResponse {
    fn from(c: crate::models::Category) -> Self {
        CategoryResponse {
            id: c.id,
            name: c.name,
            slug: c.slug,
            description: c.description,
            icon: c.icon,
            image_url: c.image_url,
            display_order: c.display_order,
            weight: None,
        }
    }
}
