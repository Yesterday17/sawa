use crate::errors::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetUserError {
    #[error("User not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("Failed to hash password: {0}")]
    FailedToHashPassword(String),

    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error("User already exists")]
    AlreadyExists,
}
