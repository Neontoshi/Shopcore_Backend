use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::SimpleAddress;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(max = 100))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100))]
    pub last_name: Option<String>,
    
    #[validate(length(max = 20))]
    pub phone: Option<String>, // Frontend uses 'phone'
    
    pub default_address: Option<SimpleAddress>,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>, // Frontend expects 'phone'
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_address: Option<SimpleAddress>,
}