use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::Category;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn create(pool: &PgPool, category: &Category) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO categories (id, name, slug, description, parent_id, icon, image_url, display_order, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            category.id,
            category.name,
            category.slug,
            category.description,
            category.parent_id,
            category.icon,
            category.image_url,
            category.display_order,
            category.is_active
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description, parent_id, icon, image_url,
                   display_order, is_active, created_at, updated_at
            FROM categories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(category)
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Category>> {
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description, parent_id, icon, image_url,
                   display_order, is_active, created_at, updated_at
            FROM categories
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;
        Ok(category)
    }

    // Used by the homepage — returns all active categories
    pub async fn find_all_active(pool: &PgPool) -> Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description, parent_id, icon, image_url,
                   display_order, is_active, created_at, updated_at
            FROM categories
            WHERE is_active = true
            ORDER BY display_order ASC, name ASC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }

    // Paginated version for admin
    pub async fn find_all(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description, parent_id, icon, image_url,
                   display_order, is_active, created_at, updated_at
            FROM categories
            ORDER BY display_order ASC, name ASC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }

    pub async fn find_root_categories(pool: &PgPool) -> Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description, parent_id, icon, image_url,
                   display_order, is_active, created_at, updated_at
            FROM categories
            WHERE parent_id IS NULL AND is_active = true
            ORDER BY display_order ASC, name ASC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }

    pub async fn find_children(pool: &PgPool, parent_id: &Uuid) -> Result<Vec<Category>> {
        let categories = sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, slug, description, parent_id, icon, image_url,
                   display_order, is_active, created_at, updated_at
            FROM categories
            WHERE parent_id = $1
            ORDER BY display_order ASC, name ASC
            "#,
            parent_id
        )
        .fetch_all(pool)
        .await?;
        Ok(categories)
    }

    pub async fn update(pool: &PgPool, category: &Category) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE categories
            SET name = $1, slug = $2, description = $3, parent_id = $4,
                icon = $5, image_url = $6, display_order = $7, is_active = $8,
                updated_at = NOW()
            WHERE id = $9
            "#,
            category.name,
            category.slug,
            category.description,
            category.parent_id,
            category.icon,
            category.image_url,
            category.display_order,
            category.is_active,
            category.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM categories WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}