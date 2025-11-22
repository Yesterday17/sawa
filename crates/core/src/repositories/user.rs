use crate::{
    errors::RepositoryError,
    models::user::{Email, User, UserId, UserUpdate, Username},
};

/// Repository for the User aggregate.
///
/// In DDD, repositories provide collection-like interface for aggregates.
/// Only aggregate roots should have repositories.
pub trait UserRepository: Send + Sync + 'static {
    /// Find a user by their unique ID.
    fn find_by_id(
        &self,
        id: &UserId,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    /// Find a user by their email address.
    fn find_by_email(
        &self,
        email: &Email,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    /// Find a user by their username.
    fn find_by_username(
        &self,
        username: &Username,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    /// Create a new user.
    fn create(&self, user: User) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    /// Update an existing user.
    fn update(
        &self,
        user: UserUpdate,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    /// Delete a user by their ID.
    ///
    /// Note: Consider soft deletion in production systems.
    fn delete(&self, id: &UserId) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
