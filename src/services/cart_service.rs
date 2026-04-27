use crate::models::{CartWithItems, CartItemWithProduct};
use crate::repositories::{CartRepository, ProductRepository};
use crate::dtos::{AddToCartRequest, UpdateCartRequest, CartResponse, CartItemResponse};
use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;

pub struct CartService;

impl CartService {
    pub async fn get_cart(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<CartWithItems, AppError> {
        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;
        let items = CartRepository::get_cart_with_items(pool, &cart.id).await?;
        let total_items: i32 = items.iter().map(|i| i.quantity).sum();
        let subtotal = items.iter()
            .fold(Decimal::new(0, 0), |acc, item| acc + item.total.unwrap_or_default());
        Ok(CartWithItems {
            cart,
            items,
            total_items,
            subtotal,
        })
    }

    pub async fn add_to_cart(
        pool: &PgPool,
        user_id: &Uuid,
        req: AddToCartRequest,
    ) -> Result<CartWithItems, AppError> {
        let product = ProductRepository::find_by_id(pool, &req.product_id)
            .await?
            .ok_or_else(|| AppError::not_found("Product"))?;

        if product.stock_quantity < req.quantity {
            return Err(AppError::bad_request("Insufficient stock"));
        }

        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;

        CartRepository::add_item(
            pool,
            &cart.id,
            &req.product_id,
            req.quantity,
            product.price,
        ).await?;

        Self::get_cart(pool, user_id).await
    }

    pub async fn update_cart_item(
        pool: &PgPool,
        user_id: &Uuid,
        item_id: &Uuid,
        req: UpdateCartRequest,
    ) -> Result<CartWithItems, AppError> {
        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;

        CartRepository::update_item_quantity(
            pool,
            &cart.id,
            item_id,
            req.quantity,
        ).await?;

        Self::get_cart(pool, user_id).await
    }

    pub async fn remove_from_cart(
        pool: &PgPool,
        user_id: &Uuid,
        item_id: &Uuid,
    ) -> Result<CartWithItems, AppError> {
        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;
        CartRepository::remove_item(pool, &cart.id, item_id).await?;
        Self::get_cart(pool, user_id).await
    }

    pub async fn clear_cart(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<(), AppError> {
        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;
        let mut conn = pool.acquire().await.map_err(AppError::Database)?;
        CartRepository::clear_cart(&mut conn, &cart.id).await?;
        Ok(())
    }
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