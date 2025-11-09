use chrono::{DateTime, Utc};
use uuid::{NonNilUuid, Uuid};

use crate::models::{
    misc::Price, product::ProductInstanceId, purchase::PurchaseOrderId, user::UserId,
};

#[derive(Debug, Clone)]
pub struct UserTransaction {
    pub id: UserTransactionId,

    /// User transferring items out
    pub from_user_id: UserId,

    /// User receiving items
    pub to_user_id: UserId,

    /// Items being transferred
    pub items: Vec<ProductInstanceId>,

    /// Price (if applicable, for future use)
    pub price: Option<Price>,

    /// Transaction status
    pub status: TransactionStatus,

    /// Which purchase order this transaction is associated with
    pub source_order_id: Option<PurchaseOrderId>,

    /// The timestamp when the transaction was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the transaction was completed.
    pub completed_at: Option<DateTime<Utc>>,
    /// The timestamp when the transaction was cancelled.
    pub cancelled_at: Option<DateTime<Utc>>,
    /// The timestamp when the transaction was last updated.
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserTransactionId(pub NonNilUuid);

impl UserTransactionId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    /// The transaction is pending.
    Pending,

    /// The transaction is completed.
    Completed,

    /// The transaction is cancelled.
    Cancelled,
}
