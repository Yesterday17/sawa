use crate::models::user::UserId;

#[derive(Debug, thiserror::Error)]
pub enum FulfillOrderError {
    #[error("Permission denied: user {user_id:?} cannot fulfill this order")]
    PermissionDenied { user_id: UserId },

    #[error("Order not found")]
    OrderNotFound,

    #[error("Order is cancelled and can not be fulfilled")]
    OrderCancelled,

    #[error("Order item not in pending status")]
    ItemNotPending,

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum CancelOrderError {
    #[error("Order not found")]
    OrderNotFound,

    #[error("Permission denied: user {user_id:?} cannot cancel this order")]
    PermissionDenied { user_id: UserId },

    #[error("Order already completed")]
    OrderAlreadyCompleted,

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}
