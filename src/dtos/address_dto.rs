use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAddressRequest {
    #[validate(length(min = 1, max = 255))]
    pub address_line1: String,
    
    pub address_line2: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub city: String,
    
    #[validate(length(min = 1, max = 100))]
    pub state: String,
    
    #[validate(length(min = 1, max = 20))]
    pub postal_code: String,
    
    #[validate(length(min = 1, max = 100))]
    pub country: String,
    
    pub is_default: bool,
    pub address_type: String, // 'shipping', 'billing', or 'both'
    
    pub recipient_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
    pub delivery_instructions: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub id: Uuid,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub is_default: bool,
    pub address_type: String,
    pub recipient_name: Option<String>,
    pub phone_number: Option<String>,
    
}

// Simple address for frontend compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleAddressDto {
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}