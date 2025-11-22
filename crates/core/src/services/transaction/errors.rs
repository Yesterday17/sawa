use crate::errors::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetTransactionError {
    #[error("Transaction not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
