use sawa_core::errors::RepositoryError;
use sea_orm::{DbErr, SqlErr, TransactionError};

pub struct DatabaseError(pub sea_orm::DbErr);

impl From<DatabaseError> for RepositoryError {
    fn from(wrapped: DatabaseError) -> Self {
        if let Some(err) = wrapped.0.sql_err() {
            match err {
                SqlErr::UniqueConstraintViolation(e) => Self::Duplicated(e),
                SqlErr::ForeignKeyConstraintViolation(e) => Self::Duplicated(e),
                err => RepositoryError::Internal(err.to_string()),
            }
        } else {
            RepositoryError::Internal(wrapped.0.to_string())
        }
    }
}

impl From<TransactionError<DbErr>> for DatabaseError {
    fn from(wrapped: TransactionError<DbErr>) -> Self {
        match wrapped {
            TransactionError::Connection(e) => DatabaseError(e),
            TransactionError::Transaction(e) => DatabaseError(e),
        }
    }
}
