use crate::errors::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetTagError {
    #[error("Tag not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CreateTagError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
