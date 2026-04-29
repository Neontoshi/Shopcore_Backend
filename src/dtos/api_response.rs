use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64) -> Self {
        Self {
            data,
            total,
            page: None,
            page_size: None,
        }
    }
    
    pub fn with_pagination(mut self, page: i64, page_size: i64) -> Self {
        self.page = Some(page);
        self.page_size = Some(page_size);
        self
    }
}
