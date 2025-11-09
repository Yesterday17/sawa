/// Domain errors represent business rule violations and validation errors.
///
/// These errors occur when domain invariants are violated.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {}

/// Repository errors represent failures at the persistence layer.
///
/// These are infrastructure concerns, not domain logic.
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {}
