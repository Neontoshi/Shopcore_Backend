use sqlx::PgPool;
use uuid::Uuid;
// use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

pub struct ShippingService;

impl ShippingService {
    pub async fn get_settings(pool: &PgPool) -> Result<(f64, f64), crate::errors::AppError> {
        let settings = sqlx::query!(
            r#"
            SELECT rate_per_kg, free_shipping_threshold
            FROM shipping_settings
            WHERE is_active = true
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .fetch_one(pool)
        .await?;
        
        let rate_per_kg = settings.rate_per_kg.to_f64().unwrap_or(2.50);
        let free_threshold = settings.free_shipping_threshold.to_f64().unwrap_or(100.00);
        
        Ok((rate_per_kg, free_threshold))
    }
    
    pub async fn calculate_shipping(
        pool: &PgPool,
        cart_items: &[(Uuid, i32)],
        subtotal: f64,
    ) -> Result<(f64, bool, f64), crate::errors::AppError> {
        let (rate_per_kg, free_threshold) = Self::get_settings(pool).await?;
        
        if subtotal >= free_threshold {
            return Ok((0.0, true, 0.0));
        }
        
        let mut total_weight_kg = 0.0;
        
        for (product_id, quantity) in cart_items {
            let row = sqlx::query!(
                "SELECT COALESCE(weight, 0) as weight FROM products WHERE id = $1",
                product_id
            )
            .fetch_one(pool)
            .await?;
            
            let weight_f64 = row.weight
                .map(|d| d.to_f64().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            total_weight_kg += weight_f64 * (*quantity as f64);
        }
        
        let shipping_cost = total_weight_kg * rate_per_kg;
        
        Ok((shipping_cost, false, total_weight_kg))
    }
}
