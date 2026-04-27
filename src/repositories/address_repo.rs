use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::Address;

pub struct AddressRepository;

impl AddressRepository {
    pub async fn create_address(pool: &PgPool, address: &Address) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO addresses (
                id, user_id, address_type, is_default, address_line1, address_line2,
                city, state, postal_code, country, recipient_name, phone_number,
                email, company_name, tax_id, delivery_instructions, is_verified
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
            address.id,
            address.user_id,
            address.address_type,
            address.is_default,
            address.address_line1,
            address.address_line2,
            address.city,
            address.state,
            address.postal_code,
            address.country,
            address.recipient_name,
            address.phone_number,
            address.email,
            address.company_name,
            address.tax_id,
            address.delivery_instructions,
            address.is_verified
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<Address>> {
        let address = sqlx::query_as!(
            Address,
            r#"
            SELECT id, user_id, address_type, is_default, address_line1, address_line2,
                   city, state, postal_code, country, recipient_name, phone_number,
                   email, company_name, tax_id, delivery_instructions, is_verified,
                   created_at, updated_at, deleted_at
            FROM addresses
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(address)
    }
    
    pub async fn find_by_user(pool: &PgPool, user_id: &Uuid) -> Result<Vec<Address>> {
        let addresses = sqlx::query_as!(
            Address,
            r#"
            SELECT id, user_id, address_type, is_default, address_line1, address_line2,
                   city, state, postal_code, country, recipient_name, phone_number,
                   email, company_name, tax_id, delivery_instructions, is_verified,
                   created_at, updated_at, deleted_at
            FROM addresses
            WHERE user_id = $1 AND deleted_at IS NULL
            ORDER BY is_default DESC, created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;
        
        Ok(addresses)
    }
    
    pub async fn update_address(pool: &PgPool, address: &Address) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE addresses
            SET address_type = $1,
                is_default = $2,
                address_line1 = $3,
                address_line2 = $4,
                city = $5,
                state = $6,
                postal_code = $7,
                country = $8,
                recipient_name = $9,
                phone_number = $10,
                email = $11,
                company_name = $12,
                tax_id = $13,
                delivery_instructions = $14,
                is_verified = $15,
                updated_at = NOW()
            WHERE id = $16
            "#,
            address.address_type,
            address.is_default,
            address.address_line1,
            address.address_line2,
            address.city,
            address.state,
            address.postal_code,
            address.country,
            address.recipient_name,
            address.phone_number,
            address.email,
            address.company_name,
            address.tax_id,
            address.delivery_instructions,
            address.is_verified,
            address.id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete_address(pool: &PgPool, id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE addresses
            SET deleted_at = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
}