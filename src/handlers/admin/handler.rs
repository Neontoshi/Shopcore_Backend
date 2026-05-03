use axum::{
    extract::{State, Path, Extension, Query},
    Json,
};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;

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
    let total_revenue = revenue_result.total.unwrap_or(Default::default());

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

// Admin: Review vendor application (approve/reject)
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

    // Get the application to find the user_id
    let application = sqlx::query!(
        r#"
        SELECT user_id FROM vendor_applications WHERE id = $1
        "#,
        application_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Application"))?;

    // Start a transaction
    let mut tx = state.get_db_pool().begin().await?;

    // Update application status
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
    .execute(&mut *tx)
    .await?;

    // If approved, update user role to vendor
    if status == "approved" {
        sqlx::query!(
            r#"
            UPDATE users SET role = 'vendor', updated_at = NOW()
            WHERE id = $1
            "#,
            application.user_id
        )
        .execute(&mut *tx)
        .await?;
    }

    // Commit transaction
    tx.commit().await?;

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

// Admin: Update user status (activate/suspend)
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
            o.payment_method, o.payment_status,
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
            "payment_method": o.payment_method,
            "payment_status": o.payment_status,
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

pub async fn mark_order_paid(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }

    let order = sqlx::query!(
        "SELECT payment_method, payment_status FROM orders WHERE id = $1",
        order_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Order"))?;

    if order.payment_status == "paid" {
        return Err(AppError::bad_request("Order is already paid"));
    }

    let mut tx = state.get_db_pool().begin().await?;

    sqlx::query!(
        "UPDATE orders SET payment_status = 'paid', status = 'confirmed', updated_at = NOW() WHERE id = $1",
        order_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "UPDATE payment_transactions SET status = 'completed', updated_at = NOW() WHERE order_id = $1",
        order_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(serde_json::json!({ "message": "Order marked as paid" })))
}

// Admin: Get inventory with filters
pub async fn get_inventory(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<crate::dtos::InventoryFilter>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }
    
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let low_stock_only = params.low_stock_only.unwrap_or(false);
    let out_of_stock_only = params.out_of_stock_only.unwrap_or(false);
    
    let (inventory, total) = crate::services::InventoryService::get_inventory(
        state.get_db_pool(),
        params.vendor_id,
        low_stock_only,
        out_of_stock_only,
        params.search,
        page,
        page_size,
    ).await?;
    
    Ok(Json(serde_json::json!({
        "items": inventory,
        "total": total,
        "page": page,
        "page_size": page_size,
        "total_pages": (total as f64 / page_size as f64).ceil() as usize,
    })))
}

// Admin: Manual stock adjustment
pub async fn manual_adjust_stock(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<crate::dtos::ManualStockAdjustRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if auth_user.role != "admin" {
        return Err(AppError::forbidden("Admin access required"));
    }
    
    crate::services::InventoryService::manual_adjust_stock(
        state.get_db_pool(),
        req,
        &auth_user.user_id,
    ).await?;
    
    Ok(Json(serde_json::json!({
        "message": "Stock adjusted successfully"
    })))
}