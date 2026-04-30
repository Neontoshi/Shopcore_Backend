use axum::{extract::{State, Query}, Json, Request};
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::{require_admin, get_auth_user};
use crate::types::PaginationParams;
use crate::repositories::{UserRepository, OrderRepository, ProductRepository};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    total_users: i64,
    total_orders: i64,
    total_products: i64,
    total_revenue: String,
    pending_orders: i64,
    low_stock_products: i64,
    recent_orders: Vec<RecentOrder>,
}

#[derive(Debug, Serialize)]
pub struct RecentOrder {
    id: uuid::Uuid,
    order_number: String,
    total: String,
    status: String,
    created_at: DateTime<Utc>,
    customer_name: String,
}

#[derive(Debug, Serialize)]
pub struct SalesReport {
    period: String,
    total_revenue: String,
    total_orders: i64,
    average_order_value: String,
    top_products: Vec<TopProduct>,
    daily_breakdown: Vec<DailySales>,
}

#[derive(Debug, Serialize)]
pub struct TopProduct {
    product_id: uuid::Uuid,
    product_name: String,
    quantity_sold: i64,
    revenue: String,
}

#[derive(Debug, Serialize)]
pub struct DailySales {
    date: String,
    orders: i64,
    revenue: String,
}

#[derive(Debug, Deserialize)]
pub struct DateRangeParams {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

pub async fn admin_dashboard(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<DashboardStats>, AppError> {
    require_admin(&request).await?;
    
    let pool = state.get_db_pool();
    
    // Get counts
    let total_users = UserRepository::count_all(pool).await?;
    let total_products = sqlx::query!("SELECT COUNT(*) as count FROM products")
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);
    
    let total_orders = sqlx::query!("SELECT COUNT(*) as count FROM orders")
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);
    
    // Get total revenue
    let revenue_result = sqlx::query!(
        "SELECT COALESCE(SUM(total), 0) as total FROM orders WHERE payment_status = 'paid'"
    )
    .fetch_one(pool)
    .await?;
    
    let total_revenue = format!("${:.2}", revenue_result.total.unwrap_or(0.0));
    
    // Get pending orders
    let pending_orders = sqlx::query!(
        "SELECT COUNT(*) as count FROM orders WHERE status = 'pending'"
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0);
    
    // Get low stock products (less than 10 items)
    let low_stock = sqlx::query!(
        "SELECT COUNT(*) as count FROM products WHERE stock_quantity < 10"
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0);
    
    // Get recent orders
    let recent = sqlx::query!(
        r#"
        SELECT o.id, o.order_number, o.total, o.status, o.created_at, u.email
        FROM orders o
        JOIN users u ON o.user_id = u.id
        ORDER BY o.created_at DESC
        LIMIT 10
        "#
    )
    .fetch_all(pool)
    .await?;
    
    let recent_orders = recent
        .into_iter()
        .map(|row| RecentOrder {
            id: row.id,
            order_number: row.order_number,
            total: format!("${:.2}", row.total),
            status: row.status,
            created_at: row.created_at,
            customer_name: row.email,
        })
        .collect();
    
    Ok(Json(DashboardStats {
        total_users,
        total_orders,
        total_products,
        total_revenue,
        pending_orders,
        low_stock_products: low_stock,
        recent_orders,
    }))
}

pub async fn sales_report(
    State(state): State<AppState>,
    request: Request,
    Query(params): Query<DateRangeParams>,
) -> Result<Json<SalesReport>, AppError> {
    require_admin(&request).await?;
    
    let pool = state.get_db_pool();
    
    let end_date = params.end_date.unwrap_or_else(Utc::now);
    let start_date = params.start_date.unwrap_or_else(|| end_date - Duration::days(30));
    
    // Get sales data
    let sales_data = sqlx::query!(
        r#"
        SELECT 
            COALESCE(SUM(total), 0) as total_revenue,
            COUNT(*) as total_orders,
            COALESCE(AVG(total), 0) as avg_order_value
        FROM orders
        WHERE payment_status = 'paid'
        AND created_at BETWEEN $1 AND $2
        "#,
        start_date,
        end_date
    )
    .fetch_one(pool)
    .await?;
    
    // Get top products
    let top_products_data = sqlx::query!(
        r#"
        SELECT 
            p.id,
            p.name,
            SUM(oi.quantity) as quantity_sold,
            SUM(oi.total) as revenue
        FROM order_items oi
        JOIN products p ON oi.product_id = p.id
        JOIN orders o ON oi.order_id = o.id
        WHERE o.payment_status = 'paid'
        AND o.created_at BETWEEN $1 AND $2
        GROUP BY p.id, p.name
        ORDER BY revenue DESC
        LIMIT 10
        "#,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;
    
    let top_products = top_products_data
        .into_iter()
        .map(|row| TopProduct {
            product_id: row.id,
            product_name: row.name,
            quantity_sold: row.quantity_sold.unwrap_or(0),
            revenue: format!("${:.2}", row.revenue.unwrap_or(0.0)),
        })
        .collect();
    
    // Get daily breakdown
    let daily_data = sqlx::query!(
        r#"
        SELECT 
            DATE(created_at) as date,
            COUNT(*) as orders,
            COALESCE(SUM(total), 0) as revenue
        FROM orders
        WHERE payment_status = 'paid'
        AND created_at BETWEEN $1 AND $2
        GROUP BY DATE(created_at)
        ORDER BY date DESC
        "#,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;
    
    let daily_breakdown = daily_data
        .into_iter()
        .map(|row| DailySales {
            date: row.date.format("%Y-%m-%d").to_string(),
            orders: row.orders,
            revenue: format!("${:.2}", row.revenue),
        })
        .collect();
    
    Ok(Json(SalesReport {
        period: format!("{} to {}", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d")),
        total_revenue: format!("${:.2}", sales_data.total_revenue.unwrap_or(0.0)),
        total_orders: sales_data.total_orders,
        average_order_value: format!("${:.2}", sales_data.avg_order_value.unwrap_or(0.0)),
        top_products,
        daily_breakdown,
    }))
}

pub async fn manage_users(
    State(state): State<AppState>,
    request: Request,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<crate::dtos::UserResponse>>, AppError> {
    require_admin(&request).await?;
    
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;
    
    let users = UserRepository::find_all(state.get_db_pool(), page_size as i64, offset as i64).await?;
    
    let responses = users
        .into_iter()
        .map(|user| crate::dtos::UserResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role.to_str().to_string(),
            is_active: user.is_active,
        })
        .collect();
    
    Ok(Json(responses))
}
// Admin: Get all vendor applications
pub async fn get_vendor_applications(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    require_admin(&request).await?;
    
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
pub async fn review_vendor_application(
    State(state): State<AppState>,
    request: Request,
    axum::extract::Path(application_id): axum::extract::Path<uuid::Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_admin(&request).await?;
    
    let status = req.get("status").and_then(|s| s.as_str()).ok_or_else(|| AppError::bad_request("Status is required"))?;
    
    let user_id = get_auth_user(&request)?.user_id;
    
    sqlx::query!(
        r#"
        UPDATE vendor_applications
        SET status = $1, reviewed_by = $2, reviewed_at = NOW(), updated_at = NOW()
        WHERE id = $3
        "#,
        status,
        user_id,
        application_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": format!("Application {} successfully", status)
    })))
}

// Admin: Get all users
pub async fn get_all_users(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    require_admin(&request).await?;
    
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
    request: Request,
    axum::extract::Path(user_id): axum::extract::Path<uuid::Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    require_admin(&request).await?;
    
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
pub async fn get_all_products_admin(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    require_admin(&request).await?;
    
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
pub async fn get_all_orders_admin(
    State(state): State<AppState>,
    request: Request,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    require_admin(&request).await?;
    
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
