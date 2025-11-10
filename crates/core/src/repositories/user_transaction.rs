use crate::{
    errors::RepositoryError,
    models::{
        transfer::{UserTransactionStatus, UserTransaction, UserTransactionId},
        user::UserId,
    },
};

/// Repository for the UserTransaction aggregate.
///
/// UserTransaction represents transfers of ProductInstances between users.
/// This includes both manual trades and automatic transfers from group buy orders.
pub trait UserTransactionRepository: Send + Sync + 'static {
    /// Find a transaction by its ID.
    fn find_by_id(
        &self,
        id: &UserTransactionId,
    ) -> impl Future<Output = Result<Option<UserTransaction>, RepositoryError>> + Send;

    /// Find all transactions where a user is the sender (from_user).
    ///
    /// If status is Some, only returns transactions with that status.
    /// If status is None, returns all transactions.
    fn find_by_from_user(
        &self,
        from_user_id: &UserId,
        status: Option<UserTransactionStatus>,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Find all transactions where a user is the receiver (to_user).
    ///
    /// If status is Some, only returns transactions with that status.
    /// If status is None, returns all transactions.
    fn find_by_to_user(
        &self,
        to_user_id: &UserId,
        status: Option<UserTransactionStatus>,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Save a transaction (create or update).
    fn save(
        &self,
        transaction: &UserTransaction,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    fn save_batch(
        &self,
        transactions: &[UserTransaction],
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete a transaction by its ID.
    ///
    /// Note: Deleting transactions should be rare. Consider cancellation instead.
    fn delete(
        &self,
        id: &UserTransactionId,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
