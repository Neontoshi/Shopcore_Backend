use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Address {
    pub id: Uuid,
    pub user_id: Uuid,
    pub address_type: String, // 'shipping', 'billing', 'both'
    pub is_default: bool,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub recipient_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
    pub delivery_instructions: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Simplified address for frontend (matches ShippingAddress type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleAddress {
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

impl From<Address> for SimpleAddress {
    fn from(addr: Address) -> Self {
        SimpleAddress {
            full_name: addr.recipient_name,
            phone: addr.phone_number,
            line1: addr.address_line1,
            line2: addr.address_line2,
            city: addr.city,
            state: addr.state,
            postal_code: addr.postal_code,
            country: addr.country,
        }
    }
}