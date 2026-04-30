// Error code constants
pub mod ErrorCode {
    // Auth errors
    pub const INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
    pub const ACCOUNT_DEACTIVATED: &str = "ACCOUNT_DEACTIVATED";
    pub const INVALID_TOKEN: &str = "INVALID_TOKEN";
    pub const MISSING_AUTH: &str = "MISSING_AUTHORIZATION";
    
    // Product errors
    pub const PRODUCT_NOT_FOUND: &str = "PRODUCT_NOT_FOUND";
    pub const PRODUCT_HAS_ORDERS: &str = "PRODUCT_HAS_ORDERS";
    pub const PRODUCT_IN_CARTS: &str = "PRODUCT_IN_CARTS";
    pub const DUPLICATE_SLUG: &str = "DUPLICATE_PRODUCT_SLUG";
    
    // Cart errors
    pub const CART_EMPTY: &str = "CART_EMPTY";
    pub const INSUFFICIENT_STOCK: &str = "INSUFFICIENT_STOCK";
    
    // Order errors
    pub const ORDER_NOT_FOUND: &str = "ORDER_NOT_FOUND";
    pub const INVALID_STATUS: &str = "INVALID_ORDER_STATUS";
    
    // Address errors
    pub const ADDRESS_NOT_FOUND: &str = "ADDRESS_NOT_FOUND";
    pub const INVALID_ADDRESS_TYPE: &str = "INVALID_ADDRESS_TYPE";
    
    // User errors
    pub const USER_NOT_FOUND: &str = "USER_NOT_FOUND";
    pub const EMAIL_EXISTS: &str = "EMAIL_ALREADY_EXISTS";
    
    // Permission errors
    pub const ACCESS_DENIED: &str = "ACCESS_DENIED";
    pub const ADMIN_REQUIRED: &str = "ADMIN_ACCESS_REQUIRED";
    pub const VENDOR_REQUIRED: &str = "VENDOR_ACCESS_REQUIRED";
    
    // Generic
    pub const BAD_REQUEST: &str = "BAD_REQUEST";
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const DATABASE_ERROR: &str = "DATABASE_ERROR";
    pub const INTERNAL_ERROR: &str = "INTERNAL_SERVER_ERROR";
}
