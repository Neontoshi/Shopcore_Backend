use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PlatformSettingsResponse {
    pub platform_fee_percent: Decimal,
    pub tax_rate: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlatformSettingsRequest {
    pub platform_fee_percent: Decimal,
    pub tax_rate: Decimal,
}
