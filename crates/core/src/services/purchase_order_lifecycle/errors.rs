#[derive(Debug, thiserror::Error)]
pub enum FulfillOrderError {
    #[error("Order not found")]
    OrderNotFound,

    #[error("Order not ready for fulfillment: {reason}")]
    NotReady { reason: String },

    #[error("Order item not in pending status")]
    ItemNotPending,

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum CancelOrderError {
    #[error("Order not found")]
    OrderNotFound,

    #[error("Order already completed")]
    OrderAlreadyCompleted,

    #[error("Repository error: {0}")]
    Repository(#[from] crate::errors::RepositoryError),
}

