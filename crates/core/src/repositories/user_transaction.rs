use crate::{
    error::RepositoryError,
    models::{
        purchase::PurchaseOrderId,
        transfer::{TransactionStatus, UserTransaction, UserTransactionId},
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

    /// Find all transactions involving a specific user (as sender or receiver).
    fn find_by_user(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Find all transactions where a user is the sender (from_user).
    fn find_by_from_user(
        &self,
        from_user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Find all transactions where a user is the receiver (to_user).
    fn find_by_to_user(
        &self,
        to_user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Find all transactions with a specific status.
    fn find_by_status(
        &self,
        status: TransactionStatus,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Find all pending transactions for a user (either as sender or receiver).
    ///
    /// This is useful for showing users their active transfers/trades.
    fn find_pending_by_user(
        &self,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Find all transactions created from a specific purchase order.
    ///
    /// This is used to track automatic transfers from group buy orders.
    fn find_by_source_order(
        &self,
        order_id: &PurchaseOrderId,
    ) -> impl Future<Output = Result<Vec<UserTransaction>, RepositoryError>> + Send;

    /// Save a transaction (create or update).
    fn save(
        &self,
        transaction: &UserTransaction,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete a transaction by its ID.
    ///
    /// Note: Deleting transactions should be rare. Consider cancellation instead.
    fn delete(
        &self,
        id: &UserTransactionId,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
