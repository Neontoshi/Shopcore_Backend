// Auth errors
pub const INVALID_CREDENTIALS: &str = "Invalid email or password";
pub const EMAIL_ALREADY_EXISTS: &str = "User with this email already exists";
pub const INVALID_TOKEN: &str = "Invalid or expired token";
pub const UNAUTHORIZED: &str = "Unauthorized access";
pub const FORBIDDEN: &str = "You don't have permission to perform this action";

// Validation errors
pub const INVALID_EMAIL: &str = "Invalid email format";
pub const PASSWORD_TOO_SHORT: &str = "Password must be at least 8 characters";
pub const PASSWORD_TOO_LONG: &str = "Password must be less than 72 characters";
pub const INVALID_PRICE: &str = "Price must be between 0 and 999999.99";
pub const INVALID_STOCK: &str = "Stock quantity cannot be negative";

// Resource errors
pub const RESOURCE_NOT_FOUND: &str = "Resource not found";
pub const PRODUCT_NOT_FOUND: &str = "Product not found";
pub const USER_NOT_FOUND: &str = "User not found";
pub const CART_NOT_FOUND: &str = "Cart not found";
pub const ORDER_NOT_FOUND: &str = "Order not found";
pub const CATEGORY_NOT_FOUND: &str = "Category not found";

// Business logic errors
pub const INSUFFICIENT_STOCK: &str = "Insufficient stock available";
pub const CART_EMPTY: &str = "Cannot create order with empty cart";
pub const INVALID_ORDER_STATUS_TRANSITION: &str = "Invalid order status transition";
pub const ALREADY_REVIEWED: &str = "You have already reviewed this product";

// Database errors
pub const DATABASE_ERROR: &str = "Database error occurred";
pub const DUPLICATE_ENTRY: &str = "Duplicate entry";
pub const FOREIGN_KEY_VIOLATION: &str = "Referenced resource does not exist";