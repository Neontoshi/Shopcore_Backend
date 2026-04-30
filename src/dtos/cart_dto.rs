use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::models::cart::{CartWithItems, CartItemWithProduct};

#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartRequest {
    #[validate(required)]
    pub product_id: Option<Uuid>,
    
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCartRequest {
    #[validate(range(min = 0, message = "Quantity must be 0 or greater"))]
    pub quantity: i32,
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

#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItemResponse>,
    pub total_items: i32,
    pub subtotal: String,
}

impl From<CartItemWithProduct> for CartItemResponse {
    fn from(item: CartItemWithProduct) -> Self {
        CartItemResponse {
            id: item.id,
            product_id: item.product_id,
            name: item.name,
            slug: item.slug,
            quantity: item.quantity,
            price: item.price.to_string(),
            total: item.total.unwrap_or_default().to_string(),
            image_url: item.image_url,
            in_stock: true,
        }
    }
}

impl From<CartWithItems> for CartResponse {
    fn from(cart: CartWithItems) -> Self {
        CartResponse {
            items: cart.items.into_iter().map(|i| i.into()).collect(),
            total_items: cart.total_items,
            subtotal: cart.subtotal.to_string(),
        }
    }
}