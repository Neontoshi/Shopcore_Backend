use sqlx::PgPool;
use uuid::Uuid;
use crate::errors::AppError;
use crate::repositories::ProductRepository;
use crate::dtos::{InventoryItemResponse, ManualStockAdjustRequest};
use crate::services::AlertService;
use crate::services::EmailService;

pub struct InventoryService;

impl InventoryService {
    pub async fn get_inventory(
        pool: &PgPool,
        vendor_id: Option<Uuid>,
        low_stock_only: bool,
        out_of_stock_only: bool,
        search: Option<String>,
        page: usize,
        page_size: usize,
    ) -> Result<(Vec<InventoryItemResponse>, i64), AppError> {
        let offset = (page - 1) * page_size;
        
        let products_with_vendor = ProductRepository::get_inventory_with_filters(
            pool,
            vendor_id,
            low_stock_only,
            out_of_stock_only,
            search.as_deref(),
            page_size as i64,
            offset as i64,
        ).await?;
        
        let total = ProductRepository::count_inventory_with_filters(
            pool,
            vendor_id,
            low_stock_only,
            out_of_stock_only,
            search.as_deref(),
        ).await?;
        
        let mut inventory_items = Vec::new();
        for (product, vendor_name) in products_with_vendor {
            let status = if product.stock_quantity <= 0 {
                "out_of_stock".to_string()
            } else if product.stock_quantity <= 5 {
                "low_stock".to_string()
            } else {
                "in_stock".to_string()
            };
            
            inventory_items.push(InventoryItemResponse {
                id: product.id,
                name: product.name,
                sku: product.sku,
                stock_quantity: product.stock_quantity,
                low_stock_threshold: 5,
                status,
                vendor_id: product.vendor_id,
                vendor_name,
                price: product.price,
                is_active: product.is_active,
                updated_at: product.updated_at,
            });
        }
        
        Ok((inventory_items, total))
    }
    
    pub async fn manual_adjust_stock(
        pool: &PgPool,
        req: ManualStockAdjustRequest,
        admin_user_id: &Uuid,
        email_service: &EmailService,
    ) -> Result<(), AppError> {
        let mut tx = pool.begin().await?;
        
        // Get current stock
        let product = ProductRepository::find_by_id_tx(&mut *tx, &req.product_id)
            .await?
            .ok_or_else(|| AppError::not_found("Product"))?;
        
        let old_quantity = product.stock_quantity;
        let new_quantity = old_quantity + req.quantity_change;
        
        if new_quantity < 0 {
            return Err(AppError::bad_request(&format!(
                "Cannot reduce stock below 0. Current stock: {}",
                old_quantity
            )));
        }
        
        // Update stock
        ProductRepository::update_stock(&mut *tx, &req.product_id, req.quantity_change).await?;
        
        // Log the change
        ProductRepository::log_inventory_change(
            &mut *tx,
            &req.product_id,
            req.quantity_change,
            old_quantity,
            &req.reason,
            None,
            Some(*admin_user_id),
        ).await?;
        
        tx.commit().await?;
        
        // Trigger low stock alert check AFTER committing
        let pool_clone = pool.clone();
        let email_service_clone = email_service.clone();
        let product_id_clone = req.product_id;
        
        tokio::spawn(async move {
            // This will check and send email if stock is low
            if let Err(e) = AlertService::trigger_low_stock_check(&pool_clone, &email_service_clone, &product_id_clone).await {
                eprintln!("Failed to trigger low stock alert: {}", e);
            }
        });
        
        Ok(())
    }
}