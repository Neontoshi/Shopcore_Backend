use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::Product;
use crate::errors::AppError;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(pool: &PgPool, product: &Product) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO products (
                id, name, slug, description, price, compare_at_price, 
                stock_quantity, category_id, sku, is_active, image_url,
                average_rating, total_reviews, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
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
            product.image_url,
            product.average_rating,
            product.total_reviews,
            product.created_at,
            product.updated_at
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT 
                id, name, slug, description, 
                price as "price: Decimal", 
                compare_at_price as "compare_at_price: Decimal", 
                stock_quantity, category_id, sku, is_active, image_url,
                average_rating as "average_rating: Decimal",
                total_reviews,
                created_at, updated_at
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(product)
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT 
                id, name, slug, description, 
                price as "price: Decimal", 
                compare_at_price as "compare_at_price: Decimal", 
                stock_quantity, category_id, sku, is_active, image_url,
                average_rating as "average_rating: Decimal",
                total_reviews,
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

    pub async fn update(pool: &PgPool, product: &Product) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE products
            SET name = $1, slug = $2, description = $3, price = $4,
                compare_at_price = $5, stock_quantity = $6, category_id = $7,
                sku = $8, is_active = $9, image_url = $10, updated_at = $11,
                average_rating = $12, total_reviews = $13
            WHERE id = $14
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
            product.updated_at,
            product.average_rating,
            product.total_reviews,
            product.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM products WHERE id = $1", id)
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
    ) -> Result<Vec<Product>, AppError> {
        let mut sql = String::from(
            "SELECT id, name, slug, description, price, compare_at_price, \
             stock_quantity, category_id, sku, is_active, image_url, \
             average_rating, total_reviews, created_at, updated_at \
             FROM products WHERE 1=1"
        );
        
        let mut params: Vec<String> = vec![];
        let mut param_count = 1;
        
        if let Some(is_active) = is_active {
            sql.push_str(&format!(" AND is_active = ${}", param_count));
            params.push(is_active.to_string());
            param_count += 1;
        }
        
        if let Some(category_id) = category_id {
            sql.push_str(&format!(" AND category_id = ${}", param_count));
            params.push(category_id.to_string());
            param_count += 1;
        }
        
        if let Some(min_price) = min_price {
            sql.push_str(&format!(" AND price >= ${}", param_count));
            params.push(min_price.to_string());
            param_count += 1;
        }
        
        if let Some(max_price) = max_price {
            sql.push_str(&format!(" AND price <= ${}", param_count));
            params.push(max_price.to_string());
            param_count += 1;
        }
        
        if let Some(query_str) = query {
            sql.push_str(&format!(" AND (name ILIKE ${} OR description ILIKE ${})", param_count, param_count));
            params.push(format!("%{}%", query_str));
            param_count += 1;
        }
        
        sql.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", param_count, param_count + 1));
        
        let mut query_builder = sqlx::query_as::<_, Product>(&sql);
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        
        let products = query_builder
            .bind(limit)
            .bind(offset)
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
    ) -> Result<i64, AppError> {
        let mut sql = String::from("SELECT COUNT(*) FROM products WHERE 1=1");
        
        if let Some(is_active) = is_active {
            sql.push_str(&format!(" AND is_active = {}", is_active));
        }
        
        if let Some(category_id) = category_id {
            sql.push_str(&format!(" AND category_id = '{}'", category_id));
        }
        
        if let Some(min_price) = min_price {
            sql.push_str(&format!(" AND price >= {}", min_price));
        }
        
        if let Some(max_price) = max_price {
            sql.push_str(&format!(" AND price <= {}", max_price));
        }
        
        if let Some(query_str) = query {
            sql.push_str(&format!(" AND (name ILIKE '%{}%' OR description ILIKE '%{}%')", query_str, query_str));
        }
        
        let count: i64 = sqlx::query_scalar(&sql)
            .fetch_one(pool)
            .await?;
        
        Ok(count)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT 
                id, name, slug, description, 
                price as "price: Decimal", 
                compare_at_price as "compare_at_price: Decimal", 
                stock_quantity, category_id, sku, is_active, image_url,
                average_rating as "average_rating: Decimal",
                total_reviews,
                created_at, updated_at
            FROM products
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(products)
    }

    // Version for use with transactions
    pub async fn update_stock<'a, E>(executor: E, product_id: &Uuid, quantity: i32) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
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

    // Version for use with transactions (accepts Executor)
    pub async fn find_by_id_tx<'a, E>(executor: E, id: &Uuid) -> Result<Option<Product>, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT 
                id, name, slug, description, 
                price as "price: Decimal", 
                compare_at_price as "compare_at_price: Decimal", 
                stock_quantity, category_id, sku, is_active, image_url,
                average_rating as "average_rating: Decimal",
                total_reviews,
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
}