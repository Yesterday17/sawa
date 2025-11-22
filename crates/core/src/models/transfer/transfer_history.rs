use chrono::{DateTime, Utc};

use crate::models::user::UserId;

crate::create_entity_id!(ProductInstanceTransferHistoryId);

#[derive(Debug, Clone)]
pub struct ProductInstanceTransferHistory {
    /// The unique identifier for this transfer history.
    pub id: ProductInstanceTransferHistoryId,

    /// The original owner
    pub from_owner_id: Option<UserId>,
    /// The original holder
    pub from_holder_id: Option<UserId>,

    /// The new owner
    pub to_owner_id: UserId,
    /// The new holder
    pub to_holder_id: UserId,

    /// Transfer reason
    pub reason: TransferReason,

    /// When the transfer happened
    pub transferred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferReason {
    ///  Initial purchase from external supplier
    Purchase,

    /// The holder is changing (e.g., item moved to a different inventory)
    Delivery,

    /// User-to-user trade
    Trade,

    /// Gift from another user
    Gift,

    /// Admin operation
    AdminTransfer,
}
