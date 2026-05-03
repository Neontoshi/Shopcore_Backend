use axum::{
    extract::{State, Extension},
    Json,
};
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use rust_decimal::Decimal;

pub async fn get_vendor_stats(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_manage_products() {
        return Err(AppError::forbidden("Only vendors can access this endpoint"));
    }

    let total_products = sqlx::query!(
        r#"SELECT COUNT(*) as count FROM products WHERE vendor_id = $1"#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    let orders_data = sqlx::query!(
        r#"
        SELECT 
            COUNT(DISTINCT o.id) as order_count,
            COALESCE(SUM(o.total), 0) as total_revenue
        FROM orders o
        JOIN order_items oi ON o.id = oi.order_id
        JOIN products p ON oi.product_id = p.id
        WHERE p.vendor_id = $1 AND o.status != 'cancelled'
        "#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?;

    let total_orders = orders_data.order_count.unwrap_or(0);
    let total_revenue = orders_data.total_revenue.unwrap_or(Decimal::ZERO);

    let pending_orders = sqlx::query!(
        r#"
        SELECT COUNT(DISTINCT o.id) as count
        FROM orders o
        JOIN order_items oi ON o.id = oi.order_id
        JOIN products p ON oi.product_id = p.id
        WHERE p.vendor_id = $1 AND o.status = 'pending'
        "#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    let low_stock = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM products
        WHERE vendor_id = $1 AND stock_quantity < 10
        "#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "total_products": total_products,
        "total_orders": total_orders,
        "total_revenue": total_revenue,
        "pending_orders": pending_orders,
        "low_stock_products": low_stock
    })))
}