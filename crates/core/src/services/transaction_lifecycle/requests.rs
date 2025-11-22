use crate::models::{product::ProductInstanceId, transfer::UserTransactionId, user::UserId};

/// Request to create a new transaction.
pub struct CreateTransactionRequest {
    pub from_user_id: UserId,
    pub to_user_id: UserId,
    pub items: Vec<ProductInstanceId>,
}

/// Request to complete a transaction.
pub struct CompleteTransactionRequest {
    pub transaction_id: UserTransactionId,
    /// The user attempting to complete the transaction (must be the receiver)
    pub user_id: UserId,
}

/// Request to cancel a transaction.
pub struct CancelTransactionRequest {
    pub transaction_id: UserTransactionId,
    /// The user attempting to cancel the transaction (can be sender or receiver)
    pub user_id: UserId,
}
