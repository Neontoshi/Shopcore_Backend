use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{Wishlist, WishlistItem};

pub struct WishlistRepository;

impl WishlistRepository {
    pub async fn add_item(pool: &PgPool, user_id: &Uuid, product_id: &Uuid) -> Result<Wishlist> {
        let item = sqlx::query_as!(
            Wishlist,
            r#"
            INSERT INTO wishlists (id, user_id, product_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, product_id) DO NOTHING
            RETURNING id, user_id, product_id, created_at
            "#,
            Uuid::new_v4(),
            user_id,
            product_id
        )
        .fetch_optional(pool)
        .await?;

        item.ok_or_else(|| anyhow::anyhow!("Failed to add to wishlist or item already exists"))
    }

    pub async fn remove_item(pool: &PgPool, user_id: &Uuid, product_id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM wishlists
            WHERE user_id = $1 AND product_id = $2
            "#,
            user_id,
            product_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_wishlist(pool: &PgPool, user_id: &Uuid) -> Result<Vec<WishlistItem>> {
        let items = sqlx::query_as!(
            WishlistItem,
            r#"
            SELECT 
                w.id,
                w.product_id,
                p.name,
                p.slug,
                p.price,
                p.compare_at_price,
                p.image_url,
                p.average_rating,
                p.total_reviews,
                w.created_at as added_at
            FROM wishlists w
            JOIN products p ON w.product_id = p.id
            WHERE w.user_id = $1 AND p.is_active = true
            ORDER BY w.created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn is_in_wishlist(pool: &PgPool, user_id: &Uuid, product_id: &Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM wishlists 
                WHERE user_id = $1 AND product_id = $2
            ) as exists
            "#,
            user_id,
            product_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    pub async fn get_wishlist_count(pool: &PgPool, user_id: &Uuid) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM wishlists
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }
}