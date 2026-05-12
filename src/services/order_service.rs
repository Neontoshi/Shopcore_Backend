use crate::constants::order_status::OrderStatus;
use crate::dtos::{CheckoutResponse, OrderItemResponse, OrderResponse};
use crate::errors::AppError;
use crate::repositories::{AddressRepository, CartRepository, OrderRepository, ProductRepository};
use crate::services::shipping_service::ShippingService;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct OrderService;

impl OrderService {
    pub async fn checkout(
        pool: &PgPool,
        user_id: &Uuid,
        shipping_address_id: Uuid,
        billing_address_id: Uuid,
        payment_method: &str,
        notes: Option<String>,
    ) -> Result<CheckoutResponse, AppError> {
        let cart = CartRepository::find_or_create_cart(pool, user_id).await?;
        let cart_items = CartRepository::get_cart_with_items(pool, &cart.id).await?;

        if cart_items.is_empty() {
            return Err(AppError::bad_request("Cannot checkout with empty cart"));
        }

        let shipping_addr = AddressRepository::find_by_id(pool, &shipping_address_id)
            .await?
            .ok_or_else(|| AppError::not_found("Shipping address"))?;

        if shipping_addr.user_id != *user_id {
            return Err(AppError::forbidden("Address does not belong to user"));
        }

        let billing_addr = AddressRepository::find_by_id(pool, &billing_address_id)
            .await?
            .ok_or_else(|| AppError::not_found("Billing address"))?;

        if billing_addr.user_id != *user_id {
            return Err(AppError::forbidden("Address does not belong to user"));
        }

        // Calculate vendor subtotal (base prices)
        let _vendor_subtotal = cart_items.iter().fold(Decimal::ZERO, |acc, item| {
            acc + item.total.unwrap_or(Decimal::ZERO)
        });

        // Fetch platform fee percentage and tax rate
        let fee_settings = sqlx::query!(
            "SELECT platform_fee_percent, tax_rate FROM platform_settings WHERE is_active = true ORDER BY created_at DESC LIMIT 1"
        )
        .fetch_one(pool)
        .await?;
        let fee_percent = fee_settings.platform_fee_percent;
        let fee_multiplier = fee_percent / Decimal::new(100, 0);
        let tax_rate = fee_settings.tax_rate;

        // Apply platform fee to each item and calculate new subtotal
        let mut fee_adjusted_items: Vec<(Decimal, Decimal)> = Vec::new();
        let subtotal = cart_items.iter().fold(Decimal::ZERO, |acc, item| {
            let original = item.total.unwrap_or(Decimal::ZERO);
            let adjusted = original * (Decimal::new(1, 0) + fee_multiplier);
            fee_adjusted_items.push((original, adjusted));
            acc + adjusted
        });

        // Prepare cart items for shipping calculation
        let shipping_items: Vec<(Uuid, i32)> = cart_items
            .iter()
            .map(|item| (item.product_id, item.quantity))
            .collect();

        let subtotal_f64 = subtotal.to_f64().unwrap_or(0.0);
        let (shipping_cost_f64, _is_free, _total_weight) =
            ShippingService::calculate_shipping(pool, &shipping_items, subtotal_f64).await?;

        let shipping_cost = Decimal::from_f64(shipping_cost_f64).unwrap_or(Decimal::ZERO);
        let tax = (subtotal + shipping_cost) * tax_rate / Decimal::new(100, 0);
        let total = subtotal + tax + shipping_cost;

        let mut tx = pool.begin().await?;

        for item in &cart_items {
            let product = ProductRepository::find_by_id_tx(&mut *tx, &item.product_id)
                .await?
                .ok_or_else(|| AppError::not_found(&format!("Product {}", item.name)))?;

            if product.stock_quantity < item.quantity {
                return Err(AppError::bad_request(&format!(
                    "Insufficient stock for product: {}. Available: {}",
                    product.name, product.stock_quantity
                )));
            }
        }

        let order_number = Self::generate_order_number();

        let order = OrderRepository::create_order(
            &mut *tx,
            user_id,
            &order_number,
            subtotal,
            tax,
            shipping_cost,
            total,
            shipping_address_id,
            billing_address_id,
            payment_method,
            notes,
        )
        .await?;

        let mut order_items = Vec::new();

        for (i, item) in cart_items.iter().enumerate() {
            let product = ProductRepository::find_by_id_tx(&mut *tx, &item.product_id)
                .await?
                .unwrap();

            let adjusted_price = if i < fee_adjusted_items.len() {
                let (_, adjusted) = fee_adjusted_items[i];
                let unit_price = adjusted / Decimal::new(item.quantity as i64, 0);
                unit_price
            } else {
                item.price
            };

            order_items.push((
                item.product_id,
                item.quantity,
                adjusted_price,
                product.name.clone(),
                product.sku,
                product.vendor_id,
            ));

            ProductRepository::update_stock(&mut *tx, &item.product_id, -item.quantity).await?;
        }

        OrderRepository::add_order_items(&mut *tx, &order.id, order_items).await?;

        tx.commit().await?;

        Ok(CheckoutResponse {
            order_id: order.id,
            order_number: order.order_number,
            subtotal: subtotal.to_string(),
            tax: tax.to_string(),
            shipping_cost: shipping_cost.to_string(),
            total: total.to_string(),
            payment_url: None,
            message: "Order placed successfully".into(),
        })
    }

    pub async fn cancel_order_with_stock_restore(
        pool: &PgPool,
        order_id: &Uuid,
        user_id: &Uuid,
        is_admin: bool,
    ) -> Result<(), AppError> {
        let mut tx = pool.begin().await?;

        let order = OrderRepository::find_by_id(&mut *tx, order_id)
            .await?
            .ok_or_else(|| AppError::not_found("Order"))?;

        if !is_admin && order.user_id != *user_id {
            return Err(AppError::forbidden(
                "You don't have permission to cancel this order",
            ));
        }

        if order.status != OrderStatus::Pending {
            return Err(AppError::bad_request(
                "Only pending orders can be cancelled",
            ));
        }

        let order_items =
            OrderRepository::get_order_items_with_quantities(&mut *tx, order_id).await?;

        for (product_id, quantity) in order_items {
            let product = sqlx::query!(
                r#"
                SELECT stock_quantity FROM products WHERE id = $1
                "#,
                product_id
            )
            .fetch_one(&mut *tx)
            .await?;

            let old_quantity = product.stock_quantity;

            ProductRepository::update_stock(&mut *tx, &product_id, quantity).await?;

            ProductRepository::log_inventory_change(
                &mut *tx,
                &product_id,
                quantity,
                old_quantity,
                "order_cancel",
                Some(*order_id),
                Some(*user_id),
            )
            .await?;
        }

        OrderRepository::update_order_status(&mut *tx, order_id, OrderStatus::Cancelled).await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn update_payment_status(
        pool: &PgPool,
        order_id: &Uuid,
        status: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE orders
            SET payment_status = $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            status,
            order_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_order(
        pool: &PgPool,
        user_id: &Uuid,
        order_id: &Uuid,
        is_admin: bool,
    ) -> Result<OrderResponse, AppError> {
        let order = OrderRepository::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| AppError::not_found("Order"))?;

        if !is_admin && order.user_id != *user_id {
            return Err(AppError::forbidden("Access denied"));
        }

        let items = OrderRepository::get_order_items(pool, order_id).await?;

        Ok(OrderResponse {
            id: order.id,
            order_number: order.order_number,
            status: order.status,
            subtotal: order.subtotal.to_string(),
            tax: order.tax.to_string(),
            shipping_cost: order.shipping_cost.to_string(),
            total: order.total.to_string(),
            payment_method: order.payment_method,
            payment_status: order.payment_status,
            created_at: order.created_at,
            items: items
                .into_iter()
                .map(|item| OrderItemResponse {
                    product_id: item.product_id,
                    product_name: item.product_name,
                    quantity: item.quantity,
                    price: item.price.to_string(),
                    total: item.total.to_string(),
                })
                .collect(),
        })
    }

    pub async fn get_user_orders(
        pool: &PgPool,
        user_id: &Uuid,
        page: usize,
        page_size: usize,
    ) -> Result<(Vec<OrderResponse>, i64), AppError> {
        let offset = (page - 1) * page_size;
        let orders =
            OrderRepository::get_orders_by_user(pool, user_id, page_size as i64, offset as i64)
                .await?;

        let total = OrderRepository::count_user_orders(pool, user_id).await?;

        let mut responses = Vec::new();
        for order in orders {
            let items = OrderRepository::get_order_items(pool, &order.id).await?;
            responses.push(OrderResponse {
                id: order.id,
                order_number: order.order_number,
                status: order.status,
                subtotal: order.subtotal.to_string(),
                tax: order.tax.to_string(),
                shipping_cost: order.shipping_cost.to_string(),
                total: order.total.to_string(),
                payment_method: order.payment_method,
                payment_status: order.payment_status,
                created_at: order.created_at,
                items: items
                    .into_iter()
                    .map(|item| OrderItemResponse {
                        product_id: item.product_id,
                        product_name: item.product_name,
                        quantity: item.quantity,
                        price: item.price.to_string(),
                        total: item.total.to_string(),
                    })
                    .collect(),
            });
        }

        Ok((responses, total))
    }

    pub async fn update_order_status(
        pool: &PgPool,
        order_id: &Uuid,
        status: OrderStatus,
    ) -> Result<(), AppError> {
        let order = OrderRepository::find_by_id(pool, order_id)
            .await?
            .ok_or_else(|| AppError::not_found("Order"))?;

        if !order.status.can_transition_to(status) {
            return Err(AppError::bad_request("Invalid status transition"));
        }

        OrderRepository::update_order_status(pool, order_id, status).await?;

        Ok(())
    }

    fn generate_order_number() -> String {
        use chrono::Utc;
        let timestamp = Utc::now().format("%Y%m%d%H%M%S");
        let random = &Uuid::new_v4().simple().to_string()[..6].to_uppercase();
        format!("ORD-{}-{}", timestamp, random)
    }
}
