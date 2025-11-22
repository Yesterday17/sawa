use crate::errors::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateTransactionError {
    #[error("One or more items not found")]
    ItemNotFound,
    #[error("One or more items do not belong to sender")]
    ItemNotOwned,
    #[error("One or more items are not in Active status")]
    ItemNotActive,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CompleteTransactionError {
    #[error("Transaction not found")]
    NotFound,
    #[error("Transaction already completed")]
    AlreadyCompleted,
    #[error("Transaction cancelled")]
    Cancelled,
    #[error("Permission denied")]
    PermissionDenied,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CancelTransactionError {
    #[error("Transaction not found")]
    NotFound,
    #[error("Transaction already completed")]
    AlreadyCompleted,
    #[error("Transaction already cancelled")]
    AlreadyCancelled,
    #[error("Permission denied")]
    PermissionDenied,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
