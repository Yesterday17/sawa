use chrono::{DateTime, Utc};
use uuid::{NonNilUuid, Uuid};

use crate::models::{
    product::ProductVariantId, purchase::PurchaseOrderId, transfer::OwnershipTransfer, user::UserId,
};

/// Represents a specific, individual item of a product variant.
///
/// For example, if "T-Shirt" is a `Product` and "Red, Size M" is a `ProductVariant`,
/// then a `ProductInstance` would be one specific red, size M t-shirt with its own unique identifier.
/// This allows tracking individual items, for example, for inventory or warranty purposes.
#[derive(Debug, Clone)]
pub struct ProductInstance {
    /// The unique identifier for this specific product instance.
    pub id: ProductInstanceId,

    /// The ID of the product variant to which this instance belongs.
    pub variant_id: ProductVariantId,

    /// The owner of this product instance.
    pub owner_id: UserId,

    /// Status of this instance
    pub status: ProductInstanceStatus,

    /// Which purchase order created this instance
    pub source_order_id: PurchaseOrderId,

    /// When this instance was created (when order was fulfilled)
    pub created_at: DateTime<Utc>,

    /// Transfer history (optional, for auditing)
    pub transfer_history: Vec<OwnershipTransfer>,

    /// Status history
    pub status_history: Vec<ProductInstanceStatusHistory>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductInstanceId(pub NonNilUuid);

impl ProductInstanceId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductInstanceStatus {
    /// Owned by user, in their inventory
    Active,

    /// Locked for a pending transaction (cannot be traded or modified)
    Locked,

    /// Consumed/used (if applicable, e.g., consumable items)
    Consumed,

    /// User can't find it physically (lost, misplaced, etc.)
    NotFound,

    /// Destroyed/deleted (terminal state)
    Destroyed,
}

#[derive(Debug, Clone)]
pub struct ProductInstanceStatusHistory {
    /// The new status after this change
    pub status: ProductInstanceStatus,

    /// When the status was changed
    pub changed_at: DateTime<Utc>,

    /// Reason for the status change
    ///
    /// This is optional and can be used to provide a reason for the status change.
    /// For example, if the instance was destroyed, the reason could be "lost, misplaced, etc.".
    pub reason: Option<String>,
}
