use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct UpdateShippingSettingsRequest {
    pub rate_per_kg: Option<Decimal>,
    pub free_shipping_threshold: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct ShippingSettingsResponse {
    pub rate_per_kg: Decimal,
    pub free_shipping_threshold: Decimal,
    pub updated_at: DateTime<Utc>,
}
