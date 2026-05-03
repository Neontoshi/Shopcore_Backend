use serde::Deserialize;
use uuid::Uuid;

pub type Id = Uuid;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl PaginationParams {
    pub fn get_page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_page_size(&self, default: i64, max: i64) -> i64 {
        self.page_size
            .unwrap_or(default)
            .min(max)
            .max(1)
    }

    pub fn offset(&self, default_page_size: i64, max_page_size: i64) -> i64 {
        let page = self.get_page();
        let page_size = self.get_page_size(default_page_size, max_page_size);
        (page - 1) * page_size
    }
}