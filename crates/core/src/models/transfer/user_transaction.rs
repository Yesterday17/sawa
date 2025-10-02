use chrono::{DateTime, Utc};
use uuid::NonNilUuid;

use crate::models::{misc::Price, product::ProductInstanceId, user::UserId};

pub struct UserTransaction {
    pub id: UserTransactionId,

    /// Seller
    pub from_user_id: UserId,

    /// Buyer
    pub to_user_id: UserId,

    /// Items being traded
    pub items: Vec<ProductInstanceId>,

    /// Price (if selling for money)
    pub price: Option<Price>,

    /// Transaction status
    pub status: TransactionStatus,

    /// The timestamp when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction was completed.
    pub completed_at: Option<DateTime<Utc>>,
    /// The timestamp when the transaction was cancelled.
    pub cancelled_at: Option<DateTime<Utc>>,
    /// The timestamp when the transaction was last updated.
    pub updated_at: DateTime<Utc>,
}

pub struct UserTransactionId(pub NonNilUuid);

pub enum TransactionStatus {
    /// The transaction is pending.
    Pending,

    /// The transaction is completed.
    Completed,

    /// The transaction is cancelled.
    Cancelled,
}
