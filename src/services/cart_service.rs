use crate::repositories::CartRepository;
use crate::models::CartWithItems;
use crate::dtos::{AddToCartRequest, UpdateCartRequest};
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
        let product_id = req.product_id.ok_or_else(|| AppError::bad_request("Product ID is required"))?;
        
        // Get product to check stock and price
        let product = sqlx::query!(
            r#"
            SELECT price, stock_quantity, name
            FROM products
            WHERE id = $1 AND is_active = true
            "#,
            product_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("Product"))?;

        if product.stock_quantity < req.quantity {
            return Err(AppError::bad_request(format!(
                "Insufficient stock. Only {} available",
                product.stock_quantity
            )));
        }

        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;

        CartRepository::add_item(
            pool,
            &cart.id,
            &product_id,
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
        
        // If quantity is 0, remove the item
        if req.quantity == 0 {
            CartRepository::remove_item(pool, &cart.id, item_id).await?;
        } else {
            // Check stock before updating
            let item = sqlx::query!(
                r#"
                SELECT ci.product_id, p.stock_quantity
                FROM cart_items ci
                JOIN products p ON ci.product_id = p.id
                WHERE ci.cart_id = $1 AND ci.id = $2
                "#,
                cart.id,
                item_id
            )
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::not_found("Cart item"))?;
            
            if item.stock_quantity < req.quantity {
                return Err(AppError::bad_request(format!(
                    "Insufficient stock. Only {} available",
                    item.stock_quantity
                )));
            }
            
            CartRepository::update_item_quantity(pool, &cart.id, item_id, req.quantity).await?;
        }

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

