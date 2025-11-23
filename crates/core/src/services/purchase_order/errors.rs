use iso_currency::Currency;

use crate::models::{
    product::ProductVariantId,
    purchase::{PurchaseOrderId, PurchaseOrderItemId},
    user::UserId,
};

#[derive(Debug, thiserror::Error)]
pub enum CreateOrderError {
    #[error("User not found: {user_id:?}")]
    UserNotFound { user_id: UserId },

    #[error("Variant not found: {variant_id:?}")]
    VariantNotFound { variant_id: ProductVariantId },

    #[error("Currency mismatch: expected {expected:?}, got {actual:?}")]
    CurrencyMismatch {
        expected: Currency,
        actual: Currency,
    },

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

    #[error("Order is not editable")]
    OrderNotEditable,

    #[error("Currency mismatch: expected {expected:?}, got {actual:?}")]
    CurrencyMismatch {
        expected: Currency,
        actual: Currency,
    },

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum SubmitMysteryBoxResultsError {
    #[error("Order not found")]
    OrderNotFound,

    #[error("Permission denied: user {user_id:?} cannot modify this order")]
    PermissionDenied { user_id: UserId },

    #[error("Order item not found: {order_item_id:?}")]
    OrderItemNotFound { order_item_id: PurchaseOrderItemId },

    #[error("Variant not found: {variant_id:?}")]
    VariantNotFound { variant_id: ProductVariantId },

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

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}
