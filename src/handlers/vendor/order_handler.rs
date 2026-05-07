use axum::{
    extract::{State, Query, Extension},
    Json,
};
use crate::app::state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::constants::order_status::OrderStatus;
use std::str::FromStr;

pub async fn get_vendor_orders(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_manage_products() {
        return Err(AppError::forbidden("Only vendors can access this endpoint"));
    }

    let page = params.get("page").and_then(|p| p.as_u64()).unwrap_or(1);
    let page_size = params.get("page_size").and_then(|p| p.as_u64()).unwrap_or(20);

    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT ON (o.id)
            o.id as order_id,
            o.order_number,
            o.status,
            o.created_at,
            u.first_name,
            u.last_name,
            u.email as customer_email,
            (
                SELECT COALESCE(SUM(oi2.total), 0)
                FROM order_items oi2
                WHERE oi2.order_id = o.id AND oi2.vendor_id = $1
            ) as vendor_total
        FROM orders o
        JOIN order_items oi ON o.id = oi.order_id AND oi.vendor_id = $1
        JOIN users u ON o.user_id = u.id
        ORDER BY o.id, o.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        auth_user.user_id,
        page_size as i64,
        ((page - 1) * page_size) as i64
    )
    .fetch_all(state.get_db_pool())
    .await?;

    let mut orders: Vec<serde_json::Value> = Vec::new();
    for row in rows {
        let items = sqlx::query!(
            r#"
            SELECT product_name, quantity, price, total
            FROM order_items
            WHERE order_id = $1 AND vendor_id = $2
            "#,
            row.order_id,
            auth_user.user_id
        )
        .fetch_all(state.get_db_pool())
        .await?;

        let items_json: Vec<serde_json::Value> = items.iter().map(|i| serde_json::json!({
            "product_name": i.product_name,
            "quantity": i.quantity,
            "price": i.price,
            "total": i.total,
        })).collect();

        orders.push(serde_json::json!({
            "order_id": row.order_id,
            "order_number": row.order_number,
            "total": row.vendor_total,
            "status": row.status,
            "created_at": row.created_at,
            "customer_name": format!("{} {}",
                row.first_name.unwrap_or_default(),
                row.last_name.unwrap_or_default()
            ).trim().to_string(),
            "customer_email": row.customer_email,
            "items": items_json
        }));
    }

    let total = sqlx::query!(
        r#"
        SELECT COUNT(DISTINCT o.id) as count
        FROM orders o
        JOIN order_items oi ON o.id = oi.order_id
        WHERE oi.vendor_id = $1
        "#,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0);

    Ok(Json(serde_json::json!({
        "data": orders,
        "total": total,
        "page": page,
        "page_size": page_size
    })))
}

pub async fn update_order_status(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    axum::extract::Path(order_id): axum::extract::Path<uuid::Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.role.can_manage_products() {
        return Err(AppError::forbidden("Only vendors can update order status"));
    }

    // Parse and validate the requested status against the enum
    let new_status_str = req
        .get("status")
        .and_then(|s| s.as_str())
        .ok_or_else(|| AppError::bad_request("Status is required"))?;

    let new_status = OrderStatus::from_str(new_status_str)
        .map_err(|_| AppError::bad_request(&format!("Invalid status: '{}'", new_status_str)))?;

    // Verify this vendor has products in the order
    let has_vendor_products = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM order_items oi
        JOIN products p ON oi.product_id = p.id
        WHERE oi.order_id = $1 AND p.vendor_id = $2
        "#,
        order_id,
        auth_user.user_id
    )
    .fetch_one(state.get_db_pool())
    .await?
    .count
    .unwrap_or(0) > 0;

    if !has_vendor_products && !auth_user.role.can_access_admin() {
        return Err(AppError::forbidden("You don't have permission to update this order"));
    }

    // Fetch current status and enforce valid transitions
    let current_status_str = sqlx::query!(
        "SELECT status FROM orders WHERE id = $1",
        order_id
    )
    .fetch_optional(state.get_db_pool())
    .await?
    .ok_or_else(|| AppError::not_found("Order"))?
    .status;

    let current_status = OrderStatus::from_str(&current_status_str)
        .map_err(|_| AppError::bad_request("Order has unrecognised status in database"))?;

    if !current_status.can_transition_to(new_status) {
        return Err(AppError::bad_request(&format!(
            "Cannot transition order from '{}' to '{}'",
            current_status, new_status
        )));
    }

    sqlx::query!(
        "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2",
        new_status.to_string(),
        order_id
    )
    .execute(state.get_db_pool())
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Order status updated successfully"
    })))
}