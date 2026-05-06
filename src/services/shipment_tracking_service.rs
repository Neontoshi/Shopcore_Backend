use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::errors::AppError;

#[derive(Debug, Serialize)]
pub struct TrackingInfo {
    pub tracking_number: String,
    pub carrier: String,
    pub tracking_url: String,
    pub status: String,
    pub shipped_at: Option<DateTime<Utc>>,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub events: Vec<TrackingEvent>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TrackingEvent {
    pub timestamp: DateTime<Utc>,
    pub location: String,
    pub status: String,
    pub description: String,
}

pub struct ShipmentTrackingService;

impl ShipmentTrackingService {
    // Add tracking number to an order
    pub async fn add_tracking(
        pool: &PgPool,
        order_id: &Uuid,
        tracking_number: &str,
        carrier: &str,
        estimated_delivery: Option<DateTime<Utc>>,
        _send_email: bool,
    ) -> Result<(), AppError> {
        let tracking_url = Self::generate_tracking_url(carrier, tracking_number);
        
        sqlx::query!(
            r#"
            UPDATE orders 
            SET tracking_number = $1,
                carrier = $2,
                tracking_url = $3,
                estimated_delivery = $4,
                shipped_at = COALESCE(shipped_at, NOW()),
                status = 'shipped',
                updated_at = NOW()
            WHERE id = $5 AND status NOT IN ('cancelled', 'delivered', 'refunded')
            "#,
            tracking_number,
            carrier,
            tracking_url,
            estimated_delivery,
            order_id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;
        
        Ok(())
    }
    
    // Mark order as delivered
    pub async fn mark_delivered(
        pool: &PgPool,
        order_id: &Uuid,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE orders 
            SET delivered_at = NOW(),
                status = 'delivered',
                updated_at = NOW()
            WHERE id = $1 AND status = 'shipped'
            "#,
            order_id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;
        
        Ok(())
    }
    
    // Get tracking info for an order
    pub async fn get_tracking_info(
        pool: &PgPool,
        order_id: &Uuid,
        user_id: &Uuid,
        is_admin: bool,
    ) -> Result<Option<TrackingInfo>, AppError> {
        let order = sqlx::query!(
            r#"
            SELECT user_id, tracking_number, carrier, tracking_url, shipped_at, 
                   estimated_delivery, delivered_at, status::text as status
            FROM orders
            WHERE id = $1
            "#,
            order_id
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)?;
        
        match order {
            Some(order) if is_admin || order.user_id == *user_id => {
                if order.tracking_number.is_none() {
                    return Ok(None);
                }
                
                // Fetch live tracking events from TrackingMore API
                let events = if let (Some(carrier), Some(tracking_num)) = (order.carrier.as_ref(), order.tracking_number.as_ref()) {
                    match ShipmentTrackingService::fetch_live_tracking_events(carrier, tracking_num).await {
                        Ok(e) => e,
                        Err(e) => {
                            tracing::warn!("Failed to fetch tracking events: {}", e);
                            vec![]
                        }
                    }
                } else {
                    vec![]
                };
                
                Ok(Some(TrackingInfo {
                    tracking_number: order.tracking_number.unwrap(),
                    carrier: order.carrier.unwrap(),
                    tracking_url: order.tracking_url.unwrap(),
                    status: order.status.unwrap_or_default(),
                    shipped_at: order.shipped_at,
                    estimated_delivery: order.estimated_delivery,
                    delivered_at: order.delivered_at,
                    events,
                }))
            }
            Some(_) => Err(AppError::forbidden("Access denied")),
            None => Err(AppError::not_found("Order not found")),
        }
    }
    
    // Generate tracking URL based on carrier
    pub fn generate_tracking_url(carrier: &str, tracking_number: &str) -> String {
        match carrier.to_lowercase().as_str() {
            "ups" => format!("https://www.ups.com/track?tracknum={}", tracking_number),
            "fedex" => format!("https://www.fedex.com/fedextrack/?trknbr={}", tracking_number),
            "usps" => format!("https://tools.usps.com/go/TrackConfirmAction?tLabels={}", tracking_number),
            "dhl" => format!("https://www.dhl.com/en/express/tracking.html?AWB={}", tracking_number),
            "amazon" => format!("https://www.amazon.com/gp/help/customer/display.html?nodeId=GXYK2BYHX9Y3XK8J&trackingId={}", tracking_number),
            _ => String::new(),
        }
    }

    // Fetch live tracking events using TrackingMore API
    pub async fn fetch_live_tracking_events(
        carrier: &str,
        tracking_number: &str,
    ) -> Result<Vec<TrackingEvent>, AppError> {
        let api = crate::services::trackingmore_api::TrackingMoreApi::new()?;
        
        // Try with carrier-specific code first
        let carrier_code = Self::map_carrier_to_trackingmore(carrier);
        let events_result = api.fetch_tracking(&carrier_code, tracking_number).await;
        
        if let Ok(events) = events_result {
            if !events.is_empty() {
                return Ok(events);
            }
        }
        
        // Fallback to auto-detection
        match api.fetch_tracking_auto(tracking_number).await {
            Ok((_, events)) => Ok(events),
            Err(e) => {
                tracing::warn!("Failed to fetch tracking from TrackingMore: {}", e);
                Ok(vec![])
            }
        }
    }

    fn map_carrier_to_trackingmore(carrier: &str) -> String {
        match carrier.to_lowercase().as_str() {
            "ups" => "ups".to_string(),
            "fedex" => "fedex".to_string(),
            "usps" => "usps".to_string(),
            "dhl" => "dhl".to_string(),
            "dhl express" => "dhl".to_string(),
            "amazon" => "amazon".to_string(),
            "ontrac" => "ontrac".to_string(),
            "lasership" => "lasership".to_string(),
            "canada post" => "canada-post".to_string(),
            "royal mail" => "royal-mail".to_string(),
            "australia post" => "australia-post".to_string(),
            "china post" => "china-post".to_string(),
            _ => carrier.to_lowercase(),
        }
    }
}
