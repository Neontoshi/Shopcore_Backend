use regex::Regex;

pub struct Validators;

impl Validators {
    pub fn validate_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }
    
    pub fn validate_phone(phone: &str) -> bool {
        let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
        phone_regex.is_match(phone)
    }
    
    pub fn validate_slug(slug: &str) -> bool {
        let slug_regex = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
        slug_regex.is_match(slug)
    }
    
    // Basic string sanitization - remove dangerous characters
    pub fn sanitize_string(input: &str) -> String {
        // Remove HTML tags and escape special characters
        let no_html = Self::strip_html_tags(input);
        let trimmed = no_html.trim();
        
        // Escape remaining dangerous characters
        trimmed
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('/', "&#x2F;")
    }
    
    // Strip HTML tags using regex
    fn strip_html_tags(input: &str) -> String {
        let tag_regex = Regex::new(r"<[^>]*>").unwrap();
        tag_regex.replace_all(input, "").to_string()
    }
    
    // Sanitize for SQL (though SQLx handles parameters, this is extra safety)
    pub fn sanitize_for_sql(input: &str) -> String {
        input
            .replace('\'', "''")
            .replace('\\', "\\\\")
    }
    
    // Validate and sanitize product name
    pub fn sanitize_product_name(name: &str) -> Result<String, String> {
        let sanitized = Self::sanitize_string(name);
        if sanitized.is_empty() {
            return Err("Product name cannot be empty".to_string());
        }
        if sanitized.len() > 255 {
            return Err("Product name too long (max 255 characters)".to_string());
        }
        Ok(sanitized)
    }
    
    // Validate and sanitize description
    pub fn sanitize_description(description: &str) -> Result<String, String> {
        // Allow some HTML for formatting but strip dangerous tags
        let mut sanitized = description.trim().to_string();
        
        // Keep basic formatting but remove script tags
        let script_regex = Regex::new(r"(?i)<script[^>]*>.*?</script>").unwrap();
        sanitized = script_regex.replace_all(&sanitized, "").to_string();
        
        let iframe_regex = Regex::new(r"(?i)<iframe[^>]*>.*?</iframe>").unwrap();
        sanitized = iframe_regex.replace_all(&sanitized, "").to_string();
        
        if sanitized.len() > 10000 {
            return Err("Description too long (max 10000 characters)".to_string());
        }
        
        Ok(sanitized)
    }
    
    // Validate and sanitize user input for comments/reviews
    pub fn sanitize_user_input(input: &str) -> Result<String, String> {
        let sanitized = Self::sanitize_string(input);
        if sanitized.len() > 1000 {
            return Err("Input too long (max 1000 characters)".to_string());
        }
        Ok(sanitized)
    }
}
