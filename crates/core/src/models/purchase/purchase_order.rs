use chrono::{DateTime, Utc};
use uuid::{NonNilUuid, Uuid};

use crate::models::{
    misc::{Address, Price},
    purchase::PurchaseOrderItem,
    user::UserId,
};

pub struct PurchaseOrder {
    pub id: PurchaseOrderId,

    /// The user who created the order
    pub creator_id: UserId,

    /// The user who will receive the items
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurchaseOrderId(pub NonNilUuid);

impl PurchaseOrderId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PurchaseOrderStatus {
    /// Order record created, waiting for user to complete details
    /// (e.g., filling in mystery box results)
    Incomplete,

    /// All details filled, instances created in user's inventory
    Completed,

    /// Order was cancelled/refunded externally, or user wants to remove this record
    /// Instances (if created) should be marked as destroyed
    Cancelled,
}
