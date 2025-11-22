use crate::models::{product::ProductInstanceId, user::UserId};
use chrono::{DateTime, Utc};

crate::create_entity_id!(UserTransactionId);

#[derive(Debug, Clone)]
pub struct UserTransaction {
    pub id: UserTransactionId,

    /// User transferring items out
    pub from_user_id: UserId,

    /// User receiving items
    pub to_user_id: UserId,

    /// Items being transferred
    pub items: Vec<ProductInstanceId>,

    /// Transaction status
    pub status: UserTransactionStatus,

    /// The timestamp when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction was completed.
    pub completed_at: Option<DateTime<Utc>>,
    /// The timestamp when the transaction was cancelled.
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserTransactionStatus {
    /// The transaction is pending.
    Pending,

    /// The transaction is completed.
    Completed,

    /// The transaction is cancelled.
    Cancelled,
}
