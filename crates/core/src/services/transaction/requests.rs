use crate::models::{transfer::UserTransactionId, user::UserId};

/// Request to get a transaction by ID.
pub struct GetTransactionRequest {
    pub user_id: UserId,
    pub transaction_id: UserTransactionId,
}
