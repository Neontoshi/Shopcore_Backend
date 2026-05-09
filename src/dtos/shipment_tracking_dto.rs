use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct AddTrackingRequest {
    pub tracking_number: String,
    pub carrier: String,
    pub estimated_delivery: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TrackingResponse {
    pub success: bool,
    pub message: String,
    pub tracking_number: Option<String>,
    pub tracking_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEstimatedDeliveryRequest {
    pub estimated_delivery: DateTime<Utc>,
}
