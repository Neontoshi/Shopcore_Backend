use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct PlatformSettingsResponse {
    pub platform_fee_percent: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlatformSettingsRequest {
    pub platform_fee_percent: Decimal,
}