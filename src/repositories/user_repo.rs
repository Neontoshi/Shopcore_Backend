use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::models::{User, Address, SimpleAddress};
use crate::constants::roles::UserRole;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &PgPool, user: &User) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, first_name, last_name, phone_number, role, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.first_name,
            user.last_name,
            user.phone_number,
            user.role.to_str(),
            user.is_active
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, first_name, last_name, phone_number,
                   role as "role: UserRole", is_active, created_at, updated_at, last_login_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, first_name, last_name, phone_number,
                   role as "role: UserRole", is_active, created_at, updated_at, last_login_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn update_last_login(pool: &PgPool, user_id: &Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET last_login_at = NOW()
            WHERE id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn update_profile(
        pool: &PgPool,
        user_id: &Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                phone_number = COALESCE($3, phone_number),
                updated_at = NOW()
            WHERE id = $4
            "#,
            first_name,
            last_name,
            phone_number,
            user_id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_default_address(pool: &PgPool, user_id: &Uuid) -> Result<Option<Address>> {
        let address = sqlx::query_as!(
            Address,
            r#"
            SELECT id, user_id, address_type, is_default, address_line1, address_line2,
                   city, state, postal_code, country, recipient_name, phone_number,
                   email, company_name, tax_id, delivery_instructions, is_verified,
                   created_at, updated_at, deleted_at
            FROM addresses
            WHERE user_id = $1 AND is_default = true AND deleted_at IS NULL
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(address)
    }
    
    pub async fn upsert_default_address(
        pool: &PgPool,
        user_id: &Uuid,
        address: &SimpleAddress,
    ) -> Result<()> {
        // First, unset any existing default addresses
        sqlx::query!(
            "UPDATE addresses SET is_default = false, updated_at = NOW() WHERE user_id = $1 AND is_default = true",
            user_id
        )
        .execute(pool)
        .await?;
        
        // Then insert the new default address
        sqlx::query!(
            r#"
            INSERT INTO addresses (
                id, user_id, address_type, is_default, address_line1, address_line2,
                city, state, postal_code, country, recipient_name, phone_number
            )
            VALUES ($1, $2, 'both', true, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            Uuid::new_v4(),
            user_id,
            address.line1,
            address.line2,
            address.city,
            address.state,
            address.postal_code,
            address.country,
            address.full_name,
            address.phone,
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn update_password(pool: &PgPool, user_id: &Uuid, password_hash: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            password_hash,
            user_id
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn find_all(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, first_name, last_name, phone_number,
                   role as "role: UserRole", is_active, created_at, updated_at, last_login_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
        
        Ok(users)
    }
    
    pub async fn count_all(pool: &PgPool) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users
            "#
        )
        .fetch_one(pool)
        .await?;
        
        Ok(result.count.unwrap_or(0))
    }
}