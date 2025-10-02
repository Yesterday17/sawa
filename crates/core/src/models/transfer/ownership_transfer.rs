use chrono::{DateTime, Utc};

use crate::models::user::UserId;

pub struct OwnershipTransfer {
    /// From which user (None if from system/purchase)
    pub from_user_id: Option<UserId>,

    /// To which user
    pub to_user_id: UserId,

    /// Transfer reason
    pub reason: TransferReason,

    /// When the transfer happened
    pub transferred_at: DateTime<Utc>,
}

pub enum TransferReason {
    /// Initial purchase from external supplier
    Purchase,

    /// User-to-user trade
    Trade,

    /// Gift from another user
    Gift,

    /// Admin operation
    AdminTransfer,
}
