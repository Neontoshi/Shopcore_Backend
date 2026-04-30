use sqlx::{PgPool, PgConnection};
use anyhow::Result;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Cart, CartItemWithProduct};

pub struct CartRepository;

impl CartRepository {
    pub async fn find_or_create_cart(pool: &PgPool, user_id: &Uuid) -> Result<Cart> {
        let existing = sqlx::query_as!(
            Cart,
            r#"
            SELECT id, user_id, created_at, updated_at, expires_at
            FROM carts
            WHERE user_id = $1 AND expires_at > NOW()
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(cart) = existing {
            Ok(cart)
        } else {
            let new_cart = sqlx::query_as!(
                Cart,
                r#"
                INSERT INTO carts (id, user_id, expires_at)
                VALUES ($1, $2, NOW() + INTERVAL '7 days')
                RETURNING id, user_id, created_at, updated_at, expires_at
                "#,
                Uuid::new_v4(),
                user_id
            )
            .fetch_one(pool)
            .await?;

            Ok(new_cart)
        }
    }

    pub async fn get_cart_with_items(pool: &PgPool, cart_id: &Uuid) -> Result<Vec<CartItemWithProduct>> {
        let items = sqlx::query_as!(
            CartItemWithProduct,
            r#"
            SELECT
                ci.id,
                ci.product_id,
                p.name,
                p.slug,
                ci.quantity,
                ci.price_at_add as price,
                (ci.quantity * ci.price_at_add) as total,
                p.image_url
            FROM cart_items ci
            JOIN products p ON ci.product_id = p.id
            WHERE ci.cart_id = $1
            "#,
            cart_id
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn add_item(
        pool: &PgPool,
        cart_id: &Uuid,
        product_id: &Uuid,
        quantity: i32,
        price: Decimal,
    ) -> Result<()> {
        let existing = sqlx::query!(
            r#"
            SELECT id, quantity
            FROM cart_items
            WHERE cart_id = $1 AND product_id = $2
            "#,
            cart_id,
            product_id
        )
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            sqlx::query!(
                r#"
                UPDATE cart_items
                SET quantity = quantity + $1, updated_at = NOW()
                WHERE cart_id = $2 AND product_id = $3
                "#,
                quantity,
                cart_id,
                product_id
            )
            .execute(pool)
            .await?;
        } else {
            sqlx::query!(
                r#"
                INSERT INTO cart_items (id, cart_id, product_id, quantity, price_at_add)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                Uuid::new_v4(),
                cart_id,
                product_id,
                quantity,
                price
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn update_item_quantity(
        pool: &PgPool,
        cart_id: &Uuid,
        item_id: &Uuid,
        quantity: i32,
    ) -> Result<()> {
        if quantity <= 0 {
            sqlx::query!(
                r#"
                DELETE FROM cart_items
                WHERE cart_id = $1 AND id = $2
                "#,
                cart_id,
                item_id
            )
            .execute(pool)
            .await?;
        } else {
            sqlx::query!(
                r#"
                UPDATE cart_items
                SET quantity = $1, updated_at = NOW()
                WHERE cart_id = $2 AND id = $3
                "#,
                quantity,
                cart_id,
                item_id
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn remove_item(pool: &PgPool, cart_id: &Uuid, item_id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM cart_items
            WHERE cart_id = $1 AND id = $2
            "#,
            cart_id,
            item_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Takes &mut PgConnection so it can be used inside transactions
    pub async fn clear_cart(conn: &mut PgConnection, cart_id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM cart_items
            WHERE cart_id = $1
            "#,
            cart_id
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get_cart_total(pool: &PgPool, cart_id: &Uuid) -> Result<Decimal> {
        let result = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(quantity * price_at_add), 0) as total
            FROM cart_items
            WHERE cart_id = $1
            "#,
            cart_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.total.unwrap_or(Decimal::new(0, 0)))
    }
}
