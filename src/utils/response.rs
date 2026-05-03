use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EmptyResponse;

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}