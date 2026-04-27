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