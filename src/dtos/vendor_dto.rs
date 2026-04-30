use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Validate)]
pub struct ApplyForVendorRequest {
    #[validate(length(min = 2, max = 255))]
    pub store_name: String,
    
    pub store_description: Option<String>,
    
    #[validate(length(min = 5))]
    pub business_address: String,
    
    pub tax_id: Option<String>,
    
    #[validate(length(min = 5))]
    pub phone_number: String,
    
    pub bank_details: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReviewVendorApplicationRequest {
    #[validate(required)]
    pub status: Option<String>, // 'approved' or 'rejected'
    
    pub admin_notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VendorProfileResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_name: String,
    pub store_description: Option<String>,
    pub business_address: String,
    pub tax_id: Option<String>,
    pub bank_details: Option<String>,
    pub store_logo_url: Option<String>,
    pub phone_number: Option<String>,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VendorApplicationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_name: String,
    pub store_description: Option<String>,
    pub business_address: String,
    pub tax_id: Option<String>,
    pub phone_number: String,
    pub bank_details: Option<String>,
    pub status: String,
    pub admin_notes: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
