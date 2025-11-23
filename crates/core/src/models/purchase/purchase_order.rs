use crate::models::{
    misc::{Address, Price},
    purchase::PurchaseOrderItem,
    user::UserId,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

crate::create_entity_id!(PurchaseOrderId);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PurchaseOrder {
    pub id: PurchaseOrderId,

    /// The user who created the order. All items without a designated owner should belong to this user.
    pub creator_id: UserId,

    /// The user who will receive the items.
    ///
    /// Items will belong to this user first after fulfillment. Then an transaction will be created
    /// automatically if items are designated for other users, or the receiver differs from the creator.
    pub receiver_id: UserId,

    /// The items being purchased
    pub items: Vec<PurchaseOrderItem>,

    /// Shipping/delivery address (if physical goods)
    pub shipping_address: Option<Address>,

    /// Total amount paid
    pub total_price: Price,

    /// Current status of the order
    pub status: PurchaseOrderStatus,

    /// The timestamp when the order was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the order was completed.
    pub completed_at: Option<DateTime<Utc>>,
    /// The timestamp when the order was cancelled.
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PurchaseOrderStatus {
    /// Order record created, waiting for user to complete details
    /// (e.g., filling in mystery box results)
    Incomplete,

    /// All details filled, instances created in user's inventory
    Fulfilled,

    /// Order was cancelled before fulfillment.
    ///
    /// This status can only be reached from Incomplete state.
    /// Completed orders cannot be cancelled.
    ///
    /// Common reasons:
    /// - User input error (wants to delete the record)
    /// - External purchase cancelled before delivery
    Cancelled,
}
