use sawa_core::errors::RepositoryError;
use sea_orm::SqlErr;

pub struct DatabaseError(pub sea_orm::DbErr);

impl From<DatabaseError> for RepositoryError {
    fn from(wrapped: DatabaseError) -> Self {
        if let Some(err) = wrapped.0.sql_err() {
            match err {
                SqlErr::UniqueConstraintViolation(e) => Self::Duplicated(e),
                SqlErr::ForeignKeyConstraintViolation(_) => todo!(),
                err => RepositoryError::Internal(err.to_string()),
            }
        } else {
            RepositoryError::Internal(wrapped.0.to_string())
        }
    }
}
