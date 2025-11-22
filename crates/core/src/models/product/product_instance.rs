use crate::models::{
    product::ProductVariantId, purchase::PurchaseOrderLineItemId,
    transfer::ProductInstanceTransferHistory, user::UserId,
};
use chrono::{DateTime, Utc};

crate::create_entity_id!(ProductInstanceId);

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

    /// The holder of this product instance.
    pub holder_id: UserId,

    /// Status of this instance
    pub status: ProductInstanceStatus,

    /// The line item that created this instance
    ///
    /// @unique constraint to prevent duplicate instances from the same order line item
    pub source_order_line_item_id: PurchaseOrderLineItemId,

    /// When this instance was created (when order was fulfilled)
    pub created_at: DateTime<Utc>,

    /// Transfer history (optional, for auditing)
    pub transfer_history: Vec<ProductInstanceTransferHistory>,

    /// Status history
    pub status_history: Vec<ProductInstanceStatusHistory>,
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

crate::create_entity_id!(ProductInstanceStatusHistoryId);

#[derive(Debug, Clone)]
pub struct ProductInstanceStatusHistory {
    /// The unique identifier for this status change.
    pub id: ProductInstanceStatusHistoryId,

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
