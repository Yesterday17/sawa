use thiserror::Error;
use crate::errors::RepositoryError;

#[derive(Debug, Error)]
pub enum GetMediaError {
    #[error("Media not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CreateMediaError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
