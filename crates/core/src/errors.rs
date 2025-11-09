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
    #[error("Duplicate entry for field '{field}': {value}")]
    Duplicated { field: String, value: String },

    /// Cannot delete due to foreign key or dependency constraints
    ///
    /// Service layer should prevent this by checking dependencies first.
    #[error("Cannot delete: has dependencies ({details})")]
    HasDependencies { details: String },

    /// Generic infrastructure failure (database connection, query error, etc.)
    ///
    /// Service layer typically just wraps this in its own error.
    #[error("Internal error: {0}")]
    Internal(String),
}
