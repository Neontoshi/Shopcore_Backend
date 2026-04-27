use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")]

pub enum UserRole {
    Admin,
    Customer,
    Vendor,
}

impl UserRole {
    pub fn can_access_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
    
    pub fn can_manage_products(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Vendor)
    }
    
    pub fn to_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Customer => "customer",
            UserRole::Vendor => "vendor",
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}