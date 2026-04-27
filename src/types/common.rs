use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Id = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub total_items: usize,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: usize, page_size: usize, total_items: usize) -> Self {
        let total_pages = (total_items + page_size - 1) / page_size;
        
        Self {
            data,
            page,
            page_size,
            total_pages,
            total_items,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }
    
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
        }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.into()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl PaginationParams {
    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(1).max(1)
    }
    
    pub fn get_page_size(&self, default: usize, max: usize) -> usize {
        self.page_size
            .unwrap_or(default)
            .min(max)
            .max(1)
    }
    
    pub fn offset(&self, default_page_size: usize, max_page_size: usize) -> usize {
        let page = self.get_page();
        let page_size = self.get_page_size(default_page_size, max_page_size);
        (page - 1) * page_size
    }
}