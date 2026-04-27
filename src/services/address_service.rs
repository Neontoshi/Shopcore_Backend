use crate::models::Address;
use crate::repositories::AddressRepository;
use crate::dtos::{CreateAddressRequest, AddressResponse};
use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AddressService;

impl AddressService {
    pub async fn create_address(
        pool: &PgPool,
        user_id: &Uuid,
        req: CreateAddressRequest,
    ) -> Result<AddressResponse, AppError> {
        // Validate address type
        if req.address_type != "shipping" && req.address_type != "billing" && req.address_type != "both" {
            return Err(AppError::validation("Invalid address type"));
        }
        
        // If this is default, unset other defaults
        if req.is_default {
            let addresses = AddressRepository::find_by_user(pool, user_id).await?;
            for addr in addresses {
                if addr.is_default {
                    let mut updated = addr;
                    updated.is_default = false;
                    AddressRepository::update_address(pool, &updated).await?;
                }
            }
        }
        
        let address = Address {
            id: Uuid::new_v4(),
            user_id: *user_id,
            address_type: req.address_type,
            is_default: req.is_default,
            address_line1: req.address_line1,
            address_line2: req.address_line2,
            city: req.city,
            state: req.state,
            postal_code: req.postal_code,
            country: req.country,
            recipient_name: req.recipient_name,
            phone_number: req.phone_number,
            email: req.email,
            company_name: req.company_name,
            tax_id: req.tax_id,
            delivery_instructions: req.delivery_instructions,
            is_verified: false, // Default to false for new addresses
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        };
        
        AddressRepository::create_address(pool, &address).await?;
        
        Ok(address.into())
    }
    
    pub async fn get_user_addresses(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<Vec<AddressResponse>, AppError> {
        let addresses = AddressRepository::find_by_user(pool, user_id).await?;
        Ok(addresses.into_iter().map(|a| a.into()).collect())
    }
    
    pub async fn update_address(
        pool: &PgPool,
        user_id: &Uuid,
        address_id: &Uuid,
        req: CreateAddressRequest,
    ) -> Result<AddressResponse, AppError> {
        let mut address = AddressRepository::find_by_id(pool, address_id)
            .await?
            .ok_or_else(|| AppError::not_found("Address"))?;
        
        if address.user_id != *user_id {
            return Err(AppError::forbidden("Address does not belong to user"));
        }
        
        // Handle default flag
        if req.is_default && !address.is_default {
            let addresses = AddressRepository::find_by_user(pool, user_id).await?;
            for addr in addresses {
                if addr.is_default && addr.id != *address_id {
                    let mut updated = addr;
                    updated.is_default = false;
                    AddressRepository::update_address(pool, &updated).await?;
                }
            }
        }
        
        address.address_type = req.address_type;
        address.is_default = req.is_default;
        address.address_line1 = req.address_line1;
        address.address_line2 = req.address_line2;
        address.city = req.city;
        address.state = req.state;
        address.postal_code = req.postal_code;
        address.country = req.country;
        address.recipient_name = req.recipient_name;
        address.phone_number = req.phone_number;
        address.email = req.email;
        address.company_name = req.company_name;
        address.tax_id = req.tax_id;
        address.delivery_instructions = req.delivery_instructions;
        // is_verified remains unchanged during update
        address.updated_at = chrono::Utc::now();
        
        AddressRepository::update_address(pool, &address).await?;
        
        Ok(address.into())
    }
    
    pub async fn delete_address(
        pool: &PgPool,
        user_id: &Uuid,
        address_id: &Uuid,
    ) -> Result<(), AppError> {
        let address = AddressRepository::find_by_id(pool, address_id)
            .await?
            .ok_or_else(|| AppError::not_found("Address"))?;
        
        if address.user_id != *user_id {
            return Err(AppError::forbidden("Address does not belong to user"));
        }
        
        AddressRepository::delete_address(pool, address_id).await?;
        
        Ok(())
    }
}

impl From<Address> for AddressResponse {
    fn from(addr: Address) -> Self {
        AddressResponse {
            id: addr.id,
            address_line1: addr.address_line1,
            address_line2: addr.address_line2,
            city: addr.city,
            state: addr.state,
            postal_code: addr.postal_code,
            country: addr.country,
            is_default: addr.is_default,
            address_type: addr.address_type,
            phone_number: addr.phone_number,
            recipient_name: addr.recipient_name,
        }
    }
}