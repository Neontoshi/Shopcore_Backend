use bcrypt::{hash, verify};
use anyhow::Context;

pub struct PasswordService;

impl PasswordService {
    pub fn hash(password: &str, cost: u32) -> anyhow::Result<String> {
        hash(password, cost)
            .context("Failed to hash password")
    }
    
    pub fn verify(password: &str, hash: &str) -> anyhow::Result<bool> {
        verify(password, hash)
            .context("Failed to verify password")
    }
    
    pub fn validate_password_strength(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
        
        if password.len() > 72 {
            return Err("Password must be less than 72 characters".to_string());
        }
        
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        
        if !has_uppercase {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        
        if !has_lowercase {
            return Err("Password must contain at least one lowercase letter".to_string());
        }
        
        if !has_digit {
            return Err("Password must contain at least one digit".to_string());
        }
        
        if !has_special {
            return Err("Password must contain at least one special character".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_hashing() {
        let password = "Test123!@#";
        let hash = PasswordService::hash(password, 4).unwrap();
        assert!(PasswordService::verify(password, &hash).unwrap());
    }
    
    #[test]
    fn test_password_strength_validation() {
        assert!(PasswordService::validate_password_strength("Weak").is_err());
        assert!(PasswordService::validate_password_strength("NoDigit!").is_err());
        assert!(PasswordService::validate_password_strength("Nolowercase123!").is_err());
        assert!(PasswordService::validate_password_strength("NOUPPERCASE123!").is_err());
        assert!(PasswordService::validate_password_strength("Valid123!@#").is_ok());
    }
}