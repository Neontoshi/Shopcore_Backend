use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AddTrackingRequest {
    pub tracking_number: String,
    pub carrier: String,
    pub estimated_delivery: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShipmentStatusRequest {
    pub status: String,
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
}

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

#[derive(Debug, Serialize)]
pub struct TrackingEvent {
    pub timestamp: DateTime<Utc>,
    pub location: String,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct ShipmentResponse {
    pub order_id: Uuid,
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub tracking_url: Option<String>,
    pub status: String,
}
