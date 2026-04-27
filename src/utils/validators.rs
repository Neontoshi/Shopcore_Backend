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
    
    pub fn sanitize_string(input: &str) -> String {
        input.trim().to_string()
    }
}