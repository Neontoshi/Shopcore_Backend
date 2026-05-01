use axum::{
    extract::{State, Path, Query, Extension},
    Json,
};
use uuid::Uuid;
use validator::Validate;
use crate::app::state::AppState;
use crate::dtos::product_dto::{CreateProductRequest, UpdateProductRequest, ProductResponse};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;

pub async fn get_my_products(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "vendor" && auth_user.role != "admin" {
        return Err(AppError::forbidden("Only vendors can access this endpoint"));
    }

    let page = params.get("page").and_then(|p| p.as_u64()).unwrap_or(1);
    let page_size = params.get("page_size").and_then(|p| p.as_u64()).unwrap_or(20);

    let rows = sqlx::query!(
        r#"
        SELECT 
            p.id, p.name, p.slug, p.description, p.price, p.compare_at_price,
            p.stock_quantity, p.category_id, p.sku, p.is_active, p.image_url,
            p.average_rating, p.total_reviews, p.created_at, p.updated_at
        FROM products p
        WHERE p.vendor_id = $1
        ORDER BY p.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        auth_user.user_id,
        page_size as i64,
        ((page - 1) * page_size) as i64
    )
    .fetch_all(state.get_db_pool())
    .await?;

    let products: Vec<ProductResponse> = rows.into_iter().map(|row| {
        ProductResponse {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            price: row.price,
            compare_at_price: row.compare_at_price,
            stock_quantity: row.stock_quantity,
            category_id: row.category_id,
            sku: row.sku,
            is_active: row.is_active,
            image_url: row.image_url,
            average_rating: Some(row.average_rating),
            total_reviews: Some(row.total_reviews),
            created_at: row.created_at,
            updated_at: row.updated_at,
            weight: None,
            category: None,
        }
    }).collect();

    let total = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM products WHERE vendor_id = $1
        "#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "data": products,
        "total": total,
        "page": page,
        "page_size": page_size
    })))
}

pub async fn create_product(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "vendor" && auth_user.role != "admin" {
        return Err(AppError::forbidden("Only vendors can create products"));
    }

    if let Err(errors) = req.validate() {
        return Err(AppError::validation(errors.to_string()));
    }

    let product_row = sqlx::query!(
        r#"
        INSERT INTO products (name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, vendor_id, image_url, weight)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url, weight, average_rating, total_reviews, created_at, updated_at
        "#,
        req.name,
        req.slug,
        req.description,
        req.price,
        req.compare_at_price,
        req.stock_quantity,
        req.category_id,
        req.sku,
        auth_user.user_id,
        req.image_url,
        req.weight,
    )
    .fetch_one(state.get_db_pool())
    .await?;

    let product = ProductResponse {
        id: product_row.id,
        name: product_row.name,
        slug: product_row.slug,
        description: product_row.description,
        price: product_row.price,
        compare_at_price: product_row.compare_at_price,
        stock_quantity: product_row.stock_quantity,
        category_id: product_row.category_id,
        sku: product_row.sku,
        is_active: product_row.is_active,
        image_url: product_row.image_url,
        average_rating: Some(product_row.average_rating),
        total_reviews: Some(product_row.total_reviews),
        created_at: product_row.created_at,
        updated_at: product_row.updated_at,
        weight: product_row.weight,
        category: None,
    };

    Ok(Json(serde_json::json!({
        "message": "Product created successfully",
        "product": product
    })))
}

pub async fn update_product(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(product_id): Path<Uuid>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let existing = sqlx::query!(
        r#"
        SELECT vendor_id FROM products WHERE id = $1
        "#,
        product_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Product"))?;

    if existing.vendor_id != Some(auth_user.user_id) && auth_user.role != "admin" {
        return Err(AppError::forbidden("You don't have permission to update this product"));
    }

    let product_row = sqlx::query!(
        r#"
        UPDATE products
        SET name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            price = COALESCE($4, price),
            compare_at_price = COALESCE($5, compare_at_price),
            stock_quantity = COALESCE($6, stock_quantity),
            category_id = COALESCE($7, category_id),
            is_active = COALESCE($8, is_active),
            image_url = COALESCE($9, image_url),
            weight = COALESCE($10, weight),
            updated_at = NOW()
        WHERE id = $11
        RETURNING id, name, slug, description, price, compare_at_price, stock_quantity, category_id, sku, is_active, image_url, weight, average_rating, total_reviews, created_at, updated_at
        "#,
        req.name,
        req.slug,
        req.description,
        req.price,
        req.compare_at_price,
        req.stock_quantity,
        req.category_id,
        req.is_active,
        req.image_url,
        req.weight,
        product_id
    )
    .fetch_one(state.get_db_pool())
    .await?;

    let product = ProductResponse {
        id: product_row.id,
        name: product_row.name,
        slug: product_row.slug,
        description: product_row.description,
        price: product_row.price,
        compare_at_price: product_row.compare_at_price,
        stock_quantity: product_row.stock_quantity,
        category_id: product_row.category_id,
        sku: product_row.sku,
        is_active: product_row.is_active,
        image_url: product_row.image_url,
        average_rating: Some(product_row.average_rating),
        total_reviews: Some(product_row.total_reviews),
        created_at: product_row.created_at,
        updated_at: product_row.updated_at,
        weight: product_row.weight,
        category: None,
    };

    Ok(Json(serde_json::json!({
        "message": "Product updated successfully",
        "product": product
    })))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let existing = sqlx::query!(
        r#"
        SELECT vendor_id FROM products WHERE id = $1
        "#,
        product_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Product"))?;

    if existing.vendor_id != Some(auth_user.user_id) && auth_user.role != "admin" {
        return Err(AppError::forbidden("You don't have permission to delete this product"));
    }

    let order_count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM order_items WHERE product_id = $1
        "#,
        product_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    if order_count > 0 {
        return Err(AppError::bad_request("This product has existing orders. Deactivate it instead."));
    }

    let cart_count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM cart_items WHERE product_id = $1
        "#,
        product_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    if cart_count > 0 {
        return Err(AppError::bad_request("This product is in customer carts. Deactivate it instead."));
    }

    sqlx::query!(
        r#"
        DELETE FROM products WHERE id = $1
        "#,
        product_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Product deleted successfully"
    })))
}