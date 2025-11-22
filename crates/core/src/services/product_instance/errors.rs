use crate::errors::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetProductInstanceError {
    #[error("Product instance not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum ListProductInstancesError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum ConsumeProductInstanceError {
    #[error("Product instance not found")]
    NotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Product instance is not active")]
    NotActive,
    #[error("Product instance is not held by owner")]
    NotHeldByOwner,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum MarkProductInstanceLostError {
    #[error("Product instance not found")]
    NotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Product instance is not active")]
    NotActive,
    #[error("Product instance is not held by owner")]
    NotHeldByOwner,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum MarkProductInstanceDestroyedError {
    #[error("Product instance not found")]
    NotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Product instance is not active")]
    NotActive,
    #[error("Product instance is not held by owner")]
    NotHeldByOwner,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
