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