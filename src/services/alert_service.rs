use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::errors::AppError;
use crate::services::EmailService;

pub struct AlertService;

impl AlertService {
    const LOW_STOCK_THRESHOLD: i32 = 5;
    
    /// Get low stock products for dashboard widget
    pub async fn get_low_stock_summary(pool: &PgPool) -> Result<Vec<serde_json::Value>, AppError> {
        let products = sqlx::query!(
            r#"
            SELECT 
                p.id, 
                p.name, 
                p.sku, 
                p.stock_quantity,
                COALESCE(vp.store_name, u.first_name || ' ' || u.last_name, 'No Vendor') as vendor_name
            FROM products p
            LEFT JOIN users u ON p.vendor_id = u.id
            LEFT JOIN vendor_profiles vp ON u.id = vp.user_id
            WHERE p.stock_quantity <= $1 AND p.is_active = true
            "#,
            Self::LOW_STOCK_THRESHOLD
        )
        .fetch_all(pool)
        .await?;
        
        Ok(products.into_iter().map(|p| {
            serde_json::json!({
                "id": p.id,
                "name": p.name,
                "sku": p.sku,
                "stock_quantity": p.stock_quantity,
                "vendor_name": p.vendor_name,
            })
        }).collect())
    }
    
    /// Trigger low stock check and send email to vendor
    pub async fn trigger_low_stock_check(
        pool: &PgPool,
        email_service: &EmailService,
        product_id: &Uuid,
    ) -> Result<(), AppError> {
        let product = sqlx::query!(
            r#"
            SELECT 
                p.id, 
                p.name, 
                p.sku, 
                p.stock_quantity, 
                p.is_active,
                p.vendor_id
            FROM products p
            WHERE p.id = $1
            "#,
            product_id
        )
        .fetch_optional(pool)
        .await?;
        
        if let Some(product) = product {
            if product.stock_quantity <= Self::LOW_STOCK_THRESHOLD && product.is_active {
                let last_alert = sqlx::query!(
                    r#"
                    SELECT sent_at FROM low_stock_alerts
                    WHERE product_id = $1 AND alert_type = 'vendor'
                    ORDER BY sent_at DESC LIMIT 1
                    "#,
                    product_id
                )
                .fetch_optional(pool)
                .await?;
                
                let should_send = match last_alert {
                    Some(alert) => {
                        let hours_since = Utc::now().signed_duration_since(alert.sent_at).num_hours();
                        hours_since >= 24
                    }
                    None => true,
                };
                
                if should_send {
                    let mut recipient = "daisisamuel23@gmail.com".to_string();
                    let mut vendor_name = "Store Owner".to_string();
                    
                    if let Some(vendor_id) = product.vendor_id {
                        let vendor_info = sqlx::query!(
                            r#"
                            SELECT u.email, COALESCE(vp.store_name, u.first_name || ' ' || u.last_name) as name
                            FROM users u
                            LEFT JOIN vendor_profiles vp ON u.id = vp.user_id
                            WHERE u.id = $1
                            "#,
                            vendor_id
                        )
                        .fetch_optional(pool)
                        .await?;
                        
                        if let Some(vendor) = vendor_info {
                            recipient = vendor.email;
                            if let Some(name) = vendor.name {
                                vendor_name = name;
                            }
                        }
                    }
                    
                    println!("📧 Sending low stock alert to vendor: {} ({})", vendor_name, recipient);
                    
                    email_service.send_low_stock_alert(
                        &recipient,
                        &product.name,
                        product.sku.as_deref(),
                        product.stock_quantity,
                        Self::LOW_STOCK_THRESHOLD,
                        Some(&vendor_name),
                    ).await?;
                    
                    sqlx::query!(
                        r#"
                        UPDATE products
                        SET last_low_stock_alert_sent_at = NOW()
                        WHERE id = $1
                        "#,
                        product_id
                    )
                    .execute(pool)
                    .await?;
                    
                    sqlx::query!(
                        r#"
                        INSERT INTO low_stock_alerts (id, product_id, alert_type, stock_at_alert, threshold)
                        VALUES ($1, $2, $3, $4, $5)
                        ON CONFLICT (product_id, alert_type) DO UPDATE
                        SET sent_at = NOW(), stock_at_alert = $4
                        "#,
                        Uuid::new_v4(),
                        product_id,
                        "vendor",
                        product.stock_quantity,
                        Self::LOW_STOCK_THRESHOLD
                    )
                    .execute(pool)
                    .await?;
                    
                    println!("✅ Low stock alert sent to {} for product: {}", recipient, product.name);
                } else {
                    println!("⏭️ Skipping alert for {} - sent within last 24 hours", product.name);
                }
            }
        }
        
        Ok(())
    }
    
    /// Check all low stock products and send alerts to vendors (for cron job)
    pub async fn check_all_low_stock_products(
        pool: &PgPool,
        email_service: &EmailService,
    ) -> Result<(), AppError> {
        let products = sqlx::query!(
            r#"
            SELECT 
                p.id, p.name, p.sku, p.stock_quantity, p.last_low_stock_alert_sent_at,
                p.vendor_id
            FROM products p
            WHERE p.stock_quantity <= $1 AND p.is_active = true
            "#,
            Self::LOW_STOCK_THRESHOLD
        )
        .fetch_all(pool)
        .await?;
        
        for product in products {
            let should_send = match product.last_low_stock_alert_sent_at {
                Some(last_sent) => {
                    let hours_since = Utc::now().signed_duration_since(last_sent).num_hours();
                    hours_since >= 24
                }
                None => true,
            };
            
            if should_send {
                let mut recipient = "daisisamuel23@gmail.com".to_string();
                let mut vendor_name = "Store Owner".to_string();
                
                if let Some(vendor_id) = product.vendor_id {
                    let vendor_info = sqlx::query!(
                        r#"
                        SELECT u.email, COALESCE(vp.store_name, u.first_name || ' ' || u.last_name) as name
                        FROM users u
                        LEFT JOIN vendor_profiles vp ON u.id = vp.user_id
                        WHERE u.id = $1
                        "#,
                        vendor_id
                    )
                    .fetch_optional(pool)
                    .await?;
                    
                    if let Some(vendor) = vendor_info {
                        recipient = vendor.email;
                        if let Some(name) = vendor.name {
                            vendor_name = name;
                        }
                    }
                }
                
                
                email_service.send_low_stock_alert(
                    &recipient,
                    &product.name,
                    product.sku.as_deref(),
                    product.stock_quantity,
                    Self::LOW_STOCK_THRESHOLD,
                    Some(&vendor_name),
                ).await?;
                
                sqlx::query!(
                    r#"
                    UPDATE products
                    SET last_low_stock_alert_sent_at = NOW()
                    WHERE id = $1
                    "#,
                    product.id
                )
                .execute(pool)
                .await?;
                
                sqlx::query!(
                    r#"
                    INSERT INTO low_stock_alerts (id, product_id, alert_type, stock_at_alert, threshold)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (product_id, alert_type) DO UPDATE
                    SET sent_at = NOW(), stock_at_alert = $4
                    "#,
                    Uuid::new_v4(),
                    product.id,
                    "vendor",
                    product.stock_quantity,
                    Self::LOW_STOCK_THRESHOLD
                )
                .execute(pool)
                .await?;
            }
        }
        
        Ok(())
    }
}