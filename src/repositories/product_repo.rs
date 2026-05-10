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
                weight,
                average_rating as "average_rating: Decimal",
                total_reviews,
                vendor_id,
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
                weight,
                average_rating as "average_rating: Decimal",
                total_reviews,
                vendor_id,
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
    sort: Option<&str>,           // <-- ADD THIS PARAMETER
    limit: i64,
    offset: i64,
) -> Result<Vec<Product>, AppError> {
    let search_pattern = query.map(|q| format!("%{}%", q));
    
    // Determine ORDER BY based on sort
    let order_clause = match sort {
        Some("popular") => "ORDER BY total_reviews DESC NULLS LAST, created_at DESC",
        Some("rating") => "ORDER BY average_rating DESC NULLS LAST, created_at DESC",
        Some("price_asc") => "ORDER BY price ASC, created_at DESC",
        Some("price_desc") => "ORDER BY price DESC, created_at DESC",
        _ => "ORDER BY created_at DESC",  // newest (default)
    };

    let sql = format!(
        r#"
        SELECT 
            id, name, slug, description, 
            price, compare_at_price, weight,
            stock_quantity, category_id, sku, is_active, image_url,
            average_rating, total_reviews, vendor_id,
            created_at, updated_at
        FROM products 
        WHERE 
            ($1::bool IS NULL OR is_active = $1)
            AND ($2::uuid IS NULL OR category_id = $2)
            AND ($3::numeric IS NULL OR price >= $3)
            AND ($4::numeric IS NULL OR price <= $4)
            AND ($5::text IS NULL OR name ILIKE '%' || $5::text || '%' OR description ILIKE '%' || $5::text || '%')
        {order_clause}
        LIMIT $6 OFFSET $7
        "#
    );

    let products = sqlx::query_as::<_, Product>(&sql)
        .bind(is_active)
        .bind(category_id)
        .bind(min_price)
        .bind(max_price)
        .bind(search_pattern)
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
        let search_pattern = query.map(|q| format!("%{}%", q));

        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!" 
            FROM products 
            WHERE 
                ($1::bool IS NULL OR is_active = $1)
                AND ($2::uuid IS NULL OR category_id = $2)
                AND ($3::numeric IS NULL OR price >= $3)
                AND ($4::numeric IS NULL OR price <= $4)
                AND (
                    $5::text IS NULL 
                    OR name ILIKE '%' || $5::text || '%' 
                    OR description ILIKE '%' || $5::text || '%'
                )
            "#,
            is_active,
            category_id as Option<Uuid>,
            min_price as Option<Decimal>,
            max_price as Option<Decimal>,
            search_pattern as Option<String>,
        )
        .fetch_one(pool)
        .await?;

        Ok(row.count)
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
                weight,
                average_rating as "average_rating: Decimal",
                total_reviews,
                vendor_id,
                created_at, updated_at
            FROM products
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(products)
    }

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
                weight,
                average_rating as "average_rating: Decimal",
                total_reviews,
                vendor_id,
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

    pub async fn log_inventory_change<'a, E>(
        executor: E,
        product_id: &Uuid,
        quantity_change: i32,
        old_quantity: i32,
        reason: &str,
        reference_id: Option<Uuid>,
        created_by: Option<Uuid>,
    ) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            r#"
            INSERT INTO inventory_logs (id, product_id, quantity_change, old_quantity, new_quantity, reason, reference_id, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            Uuid::new_v4(),
            product_id,
            quantity_change,
            old_quantity,
            old_quantity + quantity_change,
            reason,
            reference_id,
            created_by
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    
    pub async fn get_inventory_with_filters(
        pool: &PgPool,
        vendor_id: Option<Uuid>,
        low_stock_only: bool,
        out_of_stock_only: bool,
        search: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(Product, Option<String>)>, AppError> {
        let search_pattern = search.map(|s| format!("%{}%", s));
        
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT 
                p.id, p.name, p.slug, p.description, 
                p.price as "price: Decimal", 
                p.compare_at_price as "compare_at_price: Decimal", 
                p.stock_quantity, p.category_id, p.sku, p.is_active, p.image_url,
                p.weight,
                p.average_rating as "average_rating: Decimal",
                p.total_reviews,
                p.vendor_id,
                p.created_at, p.updated_at
            FROM products p
            WHERE 
                ($1::uuid IS NULL OR p.vendor_id = $1)
                AND ($2::bool IS FALSE OR p.stock_quantity <= 5 AND p.stock_quantity > 0)
                AND ($3::bool IS FALSE OR p.stock_quantity = 0)
                AND ($4::text IS NULL OR p.name ILIKE $4)
            ORDER BY p.stock_quantity ASC 
            LIMIT $5 OFFSET $6
            "#,
            vendor_id,
            low_stock_only,
            out_of_stock_only,
            search_pattern,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
        
        let mut results = Vec::new();
        for product in products {
            let vendor_name: Option<String> = if let Some(vendor_id) = product.vendor_id {
                let row = sqlx::query!(
                    r#"
                    SELECT COALESCE(vp.store_name, u.first_name || ' ' || u.last_name) as name
                    FROM users u
                    LEFT JOIN vendor_profiles vp ON u.id = vp.user_id
                    WHERE u.id = $1
                    "#,
                    vendor_id
                )
                .fetch_optional(pool)
                .await?;
                row.and_then(|r| r.name)
            } else {
                None
            };
            results.push((product, vendor_name));
        }

        Ok(results)
    }

    pub async fn count_inventory_with_filters(
        pool: &PgPool,
        vendor_id: Option<Uuid>,
        low_stock_only: bool,
        out_of_stock_only: bool,
        search: Option<&str>,
    ) -> Result<i64, AppError> {
        let search_pattern = search.map(|s| format!("%{}%", s));

        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM products p
            WHERE 
                ($1::uuid IS NULL OR p.vendor_id = $1)
                AND ($2::bool IS FALSE OR p.stock_quantity <= 5 AND p.stock_quantity > 0)
                AND ($3::bool IS FALSE OR p.stock_quantity = 0)
                AND ($4::text IS NULL OR p.name ILIKE $4)
            "#,
            vendor_id,
            low_stock_only,
            out_of_stock_only,
            search_pattern,
        )
        .fetch_one(pool)
        .await?;

        Ok(row.count
    }
    pub async fn get_product_images(pool: &PgPool,product_id: &Uuid,)
     -> Result<Vec<crate::models::ProductImage>, AppError> {
        let images = sqlx::query_as!(
            crate::models::ProductImage,
            r#"
            SELECT id, product_id, url, alt_text, display_order, is_primary, created_at
            FROM product_images
            WHERE product_id = $1
            ORDER BY display_order ASC, created_at ASC
            "#,
            product_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(images)
    }
}