use super::*;
use crate::models::transfer::UserTransaction;

/// Service for managing user transactions (Port).
///
/// This service handles operations related to item transfers between users:
/// - Retrieving transaction details
pub trait TransactionService: Send + Sync + 'static {
    /// Get a transaction by its ID.
    fn get_transaction(
        &self,
        req: GetTransactionRequest,
    ) -> impl Future<Output = Result<UserTransaction, GetTransactionError>> + Send;
}
