use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

pub struct ShippingService;

impl ShippingService {
    const RATE_PER_KG: f64 = 2.50;
    const FREE_SHIPPING_THRESHOLD: f64 = 100.00;
    
    pub async fn calculate_shipping(
        pool: &PgPool,
        cart_items: &[(Uuid, i32)],
        subtotal: f64,
    ) -> Result<(f64, bool, f64), crate::errors::AppError> {
        if subtotal >= Self::FREE_SHIPPING_THRESHOLD {
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
        
        let shipping_cost = total_weight_kg * Self::RATE_PER_KG;
        
        Ok((shipping_cost, false, total_weight_kg))
    }
}
