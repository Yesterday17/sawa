use crate::models::misc::EmptyStringError;
use iso_currency::ParseCurrencyError;
use std::num::TryFromIntError;

/// Error types for the domain layer.
///
/// # Error Organization
///
/// - **Service errors**: Defined alongside each service (e.g., FulfillOrderError)
///   - Specific to each operation
///   - Include business context (IDs, reasons)
///   - Defined in `services/{service_name}/errors.rs`
///
/// - **RepositoryError**: Generic infrastructure error
///   - Used by all repository operations
///   - Includes business-relevant errors (Duplicated, HasDependencies)
///   - Service layer can match on variants to provide specific errors
///
/// # Design Principle
///
/// Keep RepositoryError simple and generic. Service errors are where
/// business-specific error handling happens.

/// Repository errors represent failures at the persistence layer.
///
/// This is used by all repository operations (find, save, delete, etc.).
/// While generic, it includes variants with business value.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    /// Unique constraint violation (e.g., email already exists)
    ///
    /// Service layer can check the field name to provide specific errors.
    #[error("Duplicate entry found: {0}")]
    Duplicated(String),

    #[error("Failed to convert integer: {0}")]
    InvalidInt(#[from] TryFromIntError),

    #[error("Invalid currency: {0}")]
    InvalidCurrency(#[from] ParseCurrencyError),

    /// Cannot delete due to foreign key or dependency constraints
    ///
    /// Service layer should prevent this by checking dependencies first.
    #[error("Cannot delete: has dependencies ({details})")]
    HasDependencies { details: String },

    #[error("Invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error(transparent)]
    InvalidStringValue(#[from] EmptyStringError),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Generic infrastructure failure (database connection, query error, etc.)
    ///
    /// Service layer typically just wraps this in its own error.
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
