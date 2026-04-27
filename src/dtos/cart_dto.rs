use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;


#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    
    #[validate(range(min = 1, max = 999))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCartRequest {
    #[validate(range(min = 0, max = 999))]
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItemResponse>,
    pub total_items: i32,
    pub subtotal: String,
}

#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub slug: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
    pub image_url: Option<String>,
    pub in_stock: bool,
}

#[derive(Debug, Deserialize)]
pub struct CartCheckoutRequest {
    pub shipping_address_id: Uuid,
    pub billing_address_id: Uuid,
    pub payment_method: String,
    pub notes: Option<String>,
}