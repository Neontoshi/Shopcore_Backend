use axum::{
    extract::{State, Path, Extension},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use rust_decimal::Decimal;

// Admin: Get dashboard statistics
pub async fn get_stats(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let total_users = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(state.get_db_pool())
        .await?
        .count
        .unwrap_or(0);

    let total_vendors = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE role = 'vendor'")
        .fetch_one(state.get_db_pool())
        .await?
        .count
        .unwrap_or(0);

    let total_products = sqlx::query!("SELECT COUNT(*) as count FROM products")
        .fetch_one(state.get_db_pool())
        .await?
        .count
        .unwrap_or(0);

    let total_orders = sqlx::query!("SELECT COUNT(*) as count FROM orders")
        .fetch_one(state.get_db_pool())
        .await?
        .count
        .unwrap_or(0);

    let revenue_result = sqlx::query!("SELECT COALESCE(SUM(total), 0) as total FROM orders WHERE payment_status = 'paid'")
        .fetch_one(state.get_db_pool())
        .await?;
    let total_revenue = revenue_result.total.unwrap_or(Decimal::ZERO);

    let pending_apps = sqlx::query!("SELECT COUNT(*) as count FROM vendor_applications WHERE status = 'pending'")
        .fetch_one(state.get_db_pool())
        .await?
        .count
        .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "total_users": total_users,
        "total_vendors": total_vendors,
        "total_products": total_products,
        "total_orders": total_orders,
        "total_revenue": total_revenue,
        "pending_applications": pending_apps
    })))
}

// Admin: Get all vendor applications
pub async fn get_vendor_applications(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let apps = sqlx::query!(
        r#"
        SELECT 
            va.id, va.user_id, va.store_name, va.store_description,
            va.business_address, va.tax_id, va.phone_number, va.bank_details,
            va.status, va.admin_notes, va.reviewed_by, va.reviewed_at, va.created_at,
            u.email as user_email
        FROM vendor_applications va
        JOIN users u ON va.user_id = u.id
        ORDER BY va.created_at DESC
        "#
    )
    .fetch_all(state.get_db_pool())
    .await?;

    let applications = apps.into_iter().map(|app| {
        serde_json::json!({
            "id": app.id,
            "user_id": app.user_id,
            "user_email": app.user_email,
            "store_name": app.store_name,
            "store_description": app.store_description,
            "business_address": app.business_address,
            "tax_id": app.tax_id,
            "phone_number": app.phone_number,
            "bank_details": app.bank_details,
            "status": app.status,
            "admin_notes": app.admin_notes,
            "reviewed_by": app.reviewed_by,
            "reviewed_at": app.reviewed_at,
            "created_at": app.created_at,
        })
    }).collect();

    Ok(Json(applications))
}

// Admin: Review vendor application
pub async fn review_application(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(application_id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let status = req.get("status").and_then(|s| s.as_str()).ok_or_else(|| AppError::bad_request("Status is required"))?;

    sqlx::query!(
        r#"
        UPDATE vendor_applications
        SET status = $1, reviewed_by = $2, reviewed_at = NOW(), updated_at = NOW()
        WHERE id = $3
        "#,
        status,
        auth_user.user_id,
        application_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": format!("Application {} successfully", status)
    })))
}

// Admin: Get all users
pub async fn get_users(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let users = sqlx::query!(
        r#"
        SELECT id, email, first_name, last_name, role, is_active, created_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(state.get_db_pool())
    .await?;

    let user_list = users.into_iter().map(|u| {
        serde_json::json!({
            "id": u.id,
            "email": u.email,
            "first_name": u.first_name,
            "last_name": u.last_name,
            "role": u.role,
            "is_active": u.is_active,
            "created_at": u.created_at,
        })
    }).collect();

    Ok(Json(user_list))
}

// Admin: Update user status
pub async fn update_user_status(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let is_active = req.get("is_active").and_then(|v| v.as_bool()).ok_or_else(|| AppError::bad_request("is_active is required"))?;

    sqlx::query!(
        r#"
        UPDATE users SET is_active = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        is_active,
        user_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": format!("User {} successfully", if is_active { "activated" } else { "suspended" })
    })))
}

// Admin: Get all products
pub async fn get_all_products(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let products = sqlx::query!(
        r#"
        SELECT 
            p.id, p.name, p.price, p.stock_quantity, p.is_active, p.vendor_id,
            p.created_at,
            CONCAT(u.first_name, ' ', u.last_name) as vendor_name
        FROM products p
        LEFT JOIN users u ON p.vendor_id = u.id
        ORDER BY p.created_at DESC
        "#
    )
    .fetch_all(state.get_db_pool())
    .await?;

    let product_list = products.into_iter().map(|p| {
        serde_json::json!({
            "id": p.id,
            "name": p.name,
            "price": p.price,
            "stock_quantity": p.stock_quantity,
            "is_active": p.is_active,
            "vendor_id": p.vendor_id,
            "vendor_name": p.vendor_name,
            "created_at": p.created_at,
        })
    }).collect();

    Ok(Json(product_list))
}

// Admin: Get all orders
pub async fn get_all_orders(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let orders = sqlx::query!(
        r#"
        SELECT 
            o.id, o.order_number, o.total, o.status, o.created_at,
            u.email as customer_email,
            CONCAT(u.first_name, ' ', u.last_name) as customer_name
        FROM orders o
        JOIN users u ON o.user_id = u.id
        ORDER BY o.created_at DESC
        "#
    )
    .fetch_all(state.get_db_pool())
    .await?;

    let order_list = orders.into_iter().map(|o| {
        serde_json::json!({
            "id": o.id,
            "order_number": o.order_number,
            "total": o.total,
            "status": o.status,
            "created_at": o.created_at,
            "customer_email": o.customer_email,
            "customer_name": o.customer_name,
        })
    }).collect();

    Ok(Json(order_list))
}
