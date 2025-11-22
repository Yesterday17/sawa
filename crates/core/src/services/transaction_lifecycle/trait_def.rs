use super::*;
use crate::models::transfer::UserTransaction;

/// Service for managing the lifecycle of user transactions (Port).
///
/// This service handles state transitions of transactions:
/// - Creating transactions (locking items)
/// - Completing transactions (transferring ownership)
/// - Cancelling transactions
pub trait TransactionLifecycleService: Send + Sync + 'static {
    /// Create a new transaction between users.
    ///
    /// This operation:
    /// 1. Verifies all items belong to sender and are Active
    /// 2. Locks all items (sets status to Locked)
    /// 3. Creates the transaction record in Pending status
    fn create_transaction(
        &self,
        req: CreateTransactionRequest,
    ) -> impl Future<Output = Result<UserTransaction, CreateTransactionError>> + Send;

    /// Complete a transaction.
    ///
    /// This operation:
    /// 1. Verifies the transaction is pending
    /// 2. Verifies the user has permission (must be receiver)
    /// 3. Transfers ownership of all items to the receiver
    /// 4. Updates transaction status to Completed
    fn complete_transaction(
        &self,
        req: CompleteTransactionRequest,
    ) -> impl Future<Output = Result<UserTransaction, CompleteTransactionError>> + Send;

    /// Cancel a transaction.
    ///
    /// This operation:
    /// 1. Verifies the transaction is pending
    /// 2. Verifies the user has permission (sender or receiver)
    /// 3. Unlocks all items (sets status back to Active for sender)
    /// 4. Updates transaction status to Cancelled
    fn cancel_transaction(
        &self,
        req: CancelTransactionRequest,
    ) -> impl Future<Output = Result<UserTransaction, CancelTransactionError>> + Send;
}
