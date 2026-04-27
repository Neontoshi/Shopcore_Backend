use serde::Deserialize;

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

#[derive(Debug)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub page_size: usize,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total: i64, page: usize, page_size: usize) -> Self {
        Self {
            items,
            total,
            page,
            page_size,
        }
    }
    
    pub fn total_pages(&self) -> usize {
        ((self.total as usize) + self.page_size - 1) / self.page_size
    }
}