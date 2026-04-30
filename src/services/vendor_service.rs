use sqlx::PgPool;
use uuid::Uuid;
use crate::dtos::vendor_dto::{ApplyForVendorRequest, VendorApplicationResponse, VendorProfileResponse};
use crate::errors::AppError;

pub struct VendorService;

impl VendorService {
    // Apply to become a vendor
    pub async fn apply_for_vendor(
        pool: &PgPool,
        user_id: &Uuid,
        req: ApplyForVendorRequest,
    ) -> Result<VendorApplicationResponse, AppError> {
        // Check if user already has an application
        let existing = sqlx::query!(
            r#"
            SELECT id, status FROM vendor_applications WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(app) = existing {
            if app.status == "pending" {
                return Err(AppError::bad_request("You already have a pending application"));
            }
            if app.status == "approved" {
                return Err(AppError::bad_request("You are already a vendor"));
            }
        }

        // Check if user already has a vendor profile
        let vendor_profile = sqlx::query!(
            r#"
            SELECT id FROM vendor_profiles WHERE user_id = $1 AND is_approved = true
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if vendor_profile.is_some() {
            return Err(AppError::bad_request("You are already a vendor"));
        }

        // Create new application
        let application = sqlx::query_as!(
            VendorApplicationResponse,
            r#"
            INSERT INTO vendor_applications (user_id, store_name, store_description, business_address, tax_id, phone_number, bank_details, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')
            RETURNING id, user_id, store_name, store_description, business_address, tax_id, phone_number, bank_details, status, admin_notes, reviewed_by, reviewed_at, created_at
            "#,
            user_id,
            req.store_name,
            req.store_description,
            req.business_address,
            req.tax_id,
            req.phone_number,
            req.bank_details
        )
        .fetch_one(pool)
        .await?;

        Ok(application)
    }

    // Get user's application status
    pub async fn get_my_application(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<Option<VendorApplicationResponse>, AppError> {
        let application = sqlx::query_as!(
            VendorApplicationResponse,
            r#"
            SELECT id, user_id, store_name, store_description, business_address, tax_id, phone_number, bank_details, status, admin_notes, reviewed_by, reviewed_at, created_at
            FROM vendor_applications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(application)
    }

    // Get vendor profile (if approved)
    pub async fn get_vendor_profile(
        pool: &PgPool,
        user_id: &Uuid,
    ) -> Result<Option<VendorProfileResponse>, AppError> {
        let profile = sqlx::query_as!(
            VendorProfileResponse,
            r#"
            SELECT id, user_id, store_name, store_description, business_address, tax_id, bank_details, store_logo_url, phone_number, is_approved, created_at
            FROM vendor_profiles
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(profile)
    }

    // ADMIN: Get all pending applications
    pub async fn get_pending_applications(
        pool: &PgPool,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<VendorApplicationResponse>, i64), AppError> {
        let offset = (page - 1) * page_size;
        
        let applications = sqlx::query_as!(
            VendorApplicationResponse,
            r#"
            SELECT id, user_id, store_name, store_description, business_address, tax_id, phone_number, bank_details, status, admin_notes, reviewed_by, reviewed_at, created_at
            FROM vendor_applications
            WHERE status = 'pending'
            ORDER BY created_at ASC
            LIMIT $1 OFFSET $2
            "#,
            page_size,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM vendor_applications WHERE status = 'pending'
            "#
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);

        Ok((applications, total))
    }

    // ADMIN: Review application (approve/reject)
    pub async fn review_application(
        pool: &PgPool,
        application_id: &Uuid,
        admin_id: &Uuid,
        status: &str,
        admin_notes: Option<String>,
    ) -> Result<VendorApplicationResponse, AppError> {
        // Start transaction
        let mut tx = pool.begin().await?;

        // Update application - use raw query to avoid type issues
        let application = sqlx::query_as!(
            VendorApplicationResponse,
            r#"
            UPDATE vendor_applications
            SET status = $1, admin_notes = $2, reviewed_by = $3, reviewed_at = NOW(), updated_at = NOW()
            WHERE id = $4
            RETURNING id, user_id, store_name, store_description, business_address, tax_id, phone_number, bank_details, status, admin_notes, reviewed_by, reviewed_at, created_at
            "#,
            status,
            admin_notes,
            admin_id,
            application_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // If approved, create vendor profile and update user role
        if status == "approved" {
            // Create vendor profile
            sqlx::query!(
                r#"
                INSERT INTO vendor_profiles (user_id, store_name, store_description, business_address, tax_id, phone_number, bank_details, is_approved)
                VALUES ($1, $2, $3, $4, $5, $6, $7, true)
                ON CONFLICT (user_id) DO UPDATE SET
                    store_name = EXCLUDED.store_name,
                    store_description = EXCLUDED.store_description,
                    business_address = EXCLUDED.business_address,
                    tax_id = EXCLUDED.tax_id,
                    phone_number = EXCLUDED.phone_number,
                    bank_details = EXCLUDED.bank_details,
                    is_approved = true,
                    updated_at = NOW()
                "#,
                application.user_id,
                application.store_name,
                application.store_description,
                application.business_address,
                application.tax_id,
                application.phone_number,
                application.bank_details
            )
            .execute(&mut *tx)
            .await?;

            // Update user role to vendor
            sqlx::query!(
                r#"
                UPDATE users SET role = 'vendor', updated_at = NOW()
                WHERE id = $1
                "#,
                application.user_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(application)
    }
}
