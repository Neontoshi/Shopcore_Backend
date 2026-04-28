use sqlx::{PgPool, Executor, Postgres};
use anyhow::Result;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(pool: &PgPool, product: &Product) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO products (
                id, name, slug, description, price, compare_at_price, 
                stock_quantity, category_id, sku, is_active, image_url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            product.id,
            product.name,
            product.slug,
            product.description,
            product.price,
            product.compare_at_price,
            product.stock_quantity,
            product.category_id,
            product.sku,
            product.is_active,
            product.image_url
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Product>> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, slug, description, price, compare_at_price,
                   stock_quantity, category_id, sku, is_active, image_url,
                   created_at, updated_at
            FROM products
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;
        
        Ok(products)
    }
    
    pub async fn find_by_id<'e, E>(executor: E, id: &Uuid) -> Result<Option<Product>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, slug, description, price, compare_at_price,
                   stock_quantity, category_id, sku, is_active, image_url,
                   created_at, updated_at
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(executor)
        .await?;
        
        Ok(product)
    }
    
    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Product>> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, slug, description, price, compare_at_price,
                   stock_quantity, category_id, sku, is_active, image_url,
                   created_at, updated_at
            FROM products
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(product)
    }
    
    pub async fn update(pool: &PgPool, product: &Product) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE products
            SET name = $1,
                slug = $2,
                description = $3,
                price = $4,
                compare_at_price = $5,
                stock_quantity = $6,
                category_id = $7,
                sku = $8,
                is_active = $9,
                image_url = $10,
                updated_at = NOW()
            WHERE id = $11
            "#,
            product.name,
            product.slug,
            product.description,
            product.price,
            product.compare_at_price,
            product.stock_quantity,
            product.category_id,
            product.sku,
            product.is_active,
            product.image_url,
            product.id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn update_stock<'e, E>(executor: E, product_id: &Uuid, quantity: i32) -> Result<()>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query!(
            r#"
            UPDATE products
            SET stock_quantity = stock_quantity + $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            quantity,
            product_id
        )
        .execute(executor)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM products
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn search(
        pool: &PgPool,
        query: Option<&str>,
        category_id: Option<Uuid>,
        min_price: Option<Decimal>,
        max_price: Option<Decimal>,
        is_active: Option<bool>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>> {
        let mut sql = String::from(
            r#"
            SELECT id, name, slug, description, price, compare_at_price,
                   stock_quantity, category_id, sku, is_active, image_url,
                   created_at, updated_at
            FROM products
            WHERE 1=1
            "#
        );
        
        if let Some(q) = query {
            sql.push_str(&format!(" AND (name ILIKE '%{}%' OR description ILIKE '%{}%')", q, q));
        }
        
        if let Some(cat_id) = category_id {
            sql.push_str(&format!(" AND category_id = '{}'", cat_id));
        }
        
        if let Some(min) = min_price {
            sql.push_str(&format!(" AND price >= {}", min));
        }
        
        if let Some(max) = max_price {
            sql.push_str(&format!(" AND price <= {}", max));
        }
        
        if let Some(active) = is_active {
            sql.push_str(&format!(" AND is_active = {}", active));
        }
        
        sql.push_str(" ORDER BY created_at DESC");
        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
        
        let products = sqlx::query_as::<_, Product>(&sql)
            .fetch_all(pool)
            .await?;
        
        Ok(products)
    }
    
    pub async fn count_search(
        pool: &PgPool,
        query: Option<&str>,
        category_id: Option<Uuid>,
        min_price: Option<Decimal>,
        max_price: Option<Decimal>,
        is_active: Option<bool>,
    ) -> Result<i64> {
        let mut sql = String::from(
            r#"
            SELECT COUNT(*) as count
            FROM products
            WHERE 1=1
            "#
        );
        
        if let Some(q) = query {
            sql.push_str(&format!(" AND (name ILIKE '%{}%' OR description ILIKE '%{}%')", q, q));
        }
        
        if let Some(cat_id) = category_id {
            sql.push_str(&format!(" AND category_id = '{}'", cat_id));
        }
        
        if let Some(min) = min_price {
            sql.push_str(&format!(" AND price >= {}", min));
        }
        
        if let Some(max) = max_price {
            sql.push_str(&format!(" AND price <= {}", max));
        }
        
        if let Some(active) = is_active {
            sql.push_str(&format!(" AND is_active = {}", active));
        }
        
        let result = sqlx::query_as::<_, (i64,)>(&sql)
            .fetch_one(pool)
            .await?;
        
        Ok(result.0)
    }
}