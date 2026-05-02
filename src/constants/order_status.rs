use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Confirmed,  // ← add this
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

impl OrderStatus {
    pub fn can_transition_to(&self, new_status: OrderStatus) -> bool {
        use OrderStatus::*;
        match self {
            Pending => matches!(new_status, Confirmed | Processing | Cancelled),
            Confirmed => matches!(new_status, Processing | Cancelled),
            Processing => matches!(new_status, Shipped | Cancelled),
            Shipped => matches!(new_status, Delivered | Cancelled),
            Delivered => matches!(new_status, Refunded),
            Cancelled | Refunded => false,
        }
    }
    
    pub fn is_final(&self) -> bool {
        matches!(self, OrderStatus::Delivered | OrderStatus::Cancelled | OrderStatus::Refunded)
    }
}