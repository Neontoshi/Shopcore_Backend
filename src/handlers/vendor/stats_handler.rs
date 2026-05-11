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

    // FIXED: Sum only this vendor's order items, not the full order total
    let revenue_data = sqlx::query!(
        r#"
        SELECT 
            COUNT(DISTINCT oi.order_id) as order_count,
            COALESCE(SUM(oi.total), 0) as vendor_revenue
        FROM order_items oi
        JOIN orders o ON oi.order_id = o.id
        WHERE oi.vendor_id = $1 AND o.status != 'cancelled'
        "#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?;

    let total_orders = revenue_data.order_count.unwrap_or(0);
    let total_revenue = revenue_data.vendor_revenue.unwrap_or(Decimal::ZERO);

    let pending_orders = sqlx::query!(
        r#"
        SELECT COUNT(DISTINCT oi.order_id) as count
        FROM order_items oi
        JOIN orders o ON oi.order_id = o.id
        WHERE oi.vendor_id = $1 AND o.status = 'pending'
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