use crate::models::{product::ProductVariantId, purchase::PurchaseOrderId, user::UserId};

#[derive(Debug, thiserror::Error)]
pub enum CreateOrderError {
    #[error("User not found: {user_id:?}")]
    UserNotFound { user_id: UserId },

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum AddOrderItemError {
    #[error("Order not found: {order_id:?}")]
    OrderNotFound { order_id: PurchaseOrderId },

    #[error("Permission denied: user {user_id:?} cannot modify this order")]
    PermissionDenied { user_id: UserId },

    #[error("Variant not found: {variant_id:?}")]
    VariantNotFound { variant_id: ProductVariantId },

    #[error("Order already completed")]
    OrderAlreadyCompleted,

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum SubmitMysteryBoxResultsError {
    #[error("Order not found")]
    OrderNotFound,

    #[error("Permission denied: user {user_id:?} cannot modify this order")]
    PermissionDenied { user_id: UserId },

    #[error("Order item not found")]
    OrderItemNotFound,

    #[error("Not a mystery box item")]
    NotMysteryBox,

    #[error("Invalid number of variants: expected {expected}, got {actual}")]
    InvalidVariantCount { expected: u32, actual: u32 },

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum GetOrderError {
    #[error("Order not found")]
    NotFound,

    #[error("Permission denied: user {user_id:?} cannot view this order")]
    PermissionDenied { user_id: UserId },

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}
