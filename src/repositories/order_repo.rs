use sqlx::{Executor};
use anyhow::Result;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Order, OrderItem};
use crate::constants::order_status::OrderStatus;
use crate::errors::AppError;  // ADD THIS IMPORT
use sqlx::postgres::PgConnection;

pub struct OrderRepository;

impl OrderRepository {
    pub async fn create_order<'a, E>(
        executor: E,
        user_id: &Uuid,
        order_number: &str,
        subtotal: Decimal,
        tax: Decimal,
        shipping_cost: Decimal,
        total: Decimal,
        shipping_address_id: Uuid,
        billing_address_id: Uuid,
        payment_method: &str,
        notes: Option<String>,
    ) -> Result<Order>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (
                id, user_id, order_number, status, subtotal, tax, 
                shipping_cost, total, shipping_address_id, billing_address_id,
                payment_method, payment_status, notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, user_id, order_number, status as "status: OrderStatus", 
                      subtotal, tax, shipping_cost, total, shipping_address_id, 
                      billing_address_id, payment_method, payment_status, notes,
                      created_at, updated_at, completed_at,
                      tracking_number, carrier, tracking_url,
                      shipped_at, estimated_delivery, delivered_at
            "#,
            Uuid::new_v4(),
            user_id,
            order_number,
            OrderStatus::Pending as OrderStatus,
            subtotal,
            tax,
            shipping_cost,
            total,
            shipping_address_id,
            billing_address_id,
            payment_method,
            "pending",
            notes
        )
        .fetch_one(executor)
        .await?;
        
        Ok(order)
    }
    
    pub async fn add_order_items(
        executor: &mut PgConnection,
        order_id: &Uuid,
        items: Vec<(Uuid, i32, Decimal, String, Option<String>, Option<Uuid>)>, // added vendor_id
    ) -> Result<()> {
        for (product_id, quantity, price, product_name, sku, vendor_id) in items {
            sqlx::query!(
                r#"
                INSERT INTO order_items (
                    id, order_id, product_id, quantity, price, total, 
                    product_name, product_sku, vendor_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#,
                Uuid::new_v4(),
                order_id,
                product_id,
                quantity,
                price,
                price * Decimal::new(quantity as i64, 0),
                product_name,
                sku,
                vendor_id
            )
            .execute(&mut *executor)
            .await?;
        }
        Ok(())
    }

    pub async fn find_by_id<'a, E>(
        executor: E,
        order_id: &Uuid,
    ) -> Result<Option<Order>>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id, user_id, order_number, status as "status: OrderStatus",
                   subtotal, tax, shipping_cost, total, shipping_address_id,
                   billing_address_id, payment_method, payment_status, notes,
                   created_at, updated_at, completed_at,
                   tracking_number, carrier, tracking_url,
                   shipped_at, estimated_delivery, delivered_at
            FROM orders
            WHERE id = $1
            "#,
            order_id
        )
        .fetch_optional(executor)
        .await?;
        
        Ok(order)
    }
    
    pub async fn find_by_order_number<'a, E>(
        executor: E,
        order_number: &str,
    ) -> Result<Option<Order>>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id, user_id, order_number, status as "status: OrderStatus",
                   subtotal, tax, shipping_cost, total, shipping_address_id,
                   billing_address_id, payment_method, payment_status, notes,
                   created_at, updated_at, completed_at,
                   tracking_number, carrier, tracking_url,
                   shipped_at, estimated_delivery, delivered_at
            FROM orders
            WHERE order_number = $1
            "#,
            order_number
        )
        .fetch_optional(executor)
        .await?;
        
        Ok(order)
    }
    
    pub async fn get_order_items<'a, E>(
        executor: E,
        order_id: &Uuid,
    ) -> Result<Vec<OrderItem>>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let items = sqlx::query_as!(
            OrderItem,
            r#"
            SELECT id, order_id, product_id, quantity, price, total, product_name, product_sku, created_at
            FROM order_items
            WHERE order_id = $1
            ORDER BY created_at ASC
            "#,
            order_id
        )
        .fetch_all(executor)
        .await?;
        
        Ok(items)
    }
    
    pub async fn get_orders_by_user<'a, E>(
        executor: E,
        user_id: &Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT id, user_id, order_number, status as "status: OrderStatus",
                   subtotal, tax, shipping_cost, total, shipping_address_id,
                   billing_address_id, payment_method, payment_status, notes,
                   created_at, updated_at, completed_at,
                   tracking_number, carrier, tracking_url,
                   shipped_at, estimated_delivery, delivered_at
            FROM orders
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(executor)
        .await?;
        
        Ok(orders)
    }
    
    pub async fn update_order_status<'a, E>(
        executor: E,
        order_id: &Uuid,
        status: OrderStatus,
    ) -> Result<()>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let completed_at = if status == OrderStatus::Delivered {
            Some(chrono::Utc::now())
        } else {
            None
        };
        
        sqlx::query!(
            r#"
            UPDATE orders
            SET status = $1,
                completed_at = $2,
                updated_at = NOW()
            WHERE id = $3
            "#,
            status as OrderStatus,
            completed_at,
            order_id
        )
        .execute(executor)
        .await?;
        
        Ok(())
    }
    
    pub async fn count_user_orders<'a, E>(
        executor: E,
        user_id: &Uuid,
    ) -> Result<i64>
    where
        E: Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM orders
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(executor)
        .await?;
        
        Ok(result.count.unwrap_or(0))
    }

    pub async fn get_order_items_with_quantities<'a, E>(
        executor: E,
        order_id: &Uuid,
    ) -> Result<Vec<(Uuid, i32)>, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let items = sqlx::query!(
            r#"
            SELECT product_id, quantity
            FROM order_items
            WHERE order_id = $1
            "#,
            order_id
        )
        .fetch_all(executor)
        .await?;

        Ok(items.into_iter().map(|item| (item.product_id, item.quantity)).collect())
    }
}