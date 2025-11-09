use std::num::NonZeroU32;

use chrono::{DateTime, Utc};
use uuid::{NonNilUuid, Uuid};

use crate::models::{
    misc::Price,
    product::{MysteryBoxConfig, ProductInstanceId, ProductVariantId},
    transfer::UserTransactionId,
    user::UserId,
};

/// A single item in a purchase order.
#[derive(Debug, Clone)]
pub struct PurchaseOrderItem {
    pub id: PurchaseOrderItemId,

    /// The variant of the product to purchase
    pub purchased_variant_id: ProductVariantId,

    /// All items in this order item
    ///
    /// Invariants:
    /// - For regular items: SHOULD be equal to `product_variant_id` * `quantity`
    /// - For mystery boxes: length SHOULD equal `quantity * mystery_box_snapshot.count`
    ///   - Each variant SHOULD be in `mystery_box_snapshot.possible_variants` (soft requirement)
    ///
    /// Note: Validation is lenient to allow for:
    /// - Special promotions/bonuses
    /// - User errors that can be corrected later
    pub ordered_items: Vec<OrderedItem>,

    /// Item status
    pub status: PurchaseOrderItemStatus,

    /// Snapshot of mystery box config at time of purchase
    /// If Some, this item was a mystery box purchase
    /// If None, this was a regular item purchase
    pub mystery_box_snapshot: Option<MysteryBoxConfig>,

    /// How many units to purchase
    pub quantity: NonZeroU32,

    /// Price at time of order (snapshot, immutable)
    pub unit_price: Option<Price>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurchaseOrderItemId(pub NonNilUuid);

impl PurchaseOrderItemId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

/// The status of a purchase order item.
///
/// State transitions:
/// ```text
/// AwaitingInput --user-fills--> Pending --fulfill--> Fulfilled
///      ↓                          ↓
///   Cancelled                  Cancelled
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurchaseOrderItemStatus {
    /// Waiting for user to fill in `ordered_items`.
    /// Regular items skip this status and go directly to `Pending`.
    AwaitingInput,

    /// All information complete, ready to create instances.
    /// This is the "gate" status - order can only be fulfilled when ALL items are Pending.
    Pending,

    /// Instances created successfully (terminal state).
    Fulfilled,

    /// Item cancelled before fulfillment (terminal state).
    Cancelled,
}

/// A variant pending instance creation, with ownership assignment
#[derive(Debug, Clone)]
pub struct OrderedItem {
    /// Sequence number within this order item
    /// This provides a stable reference that won't change even if items are removed
    pub seq: u32,

    /// The variant to create instance for
    pub variant_id: ProductVariantId,

    /// The ultimate owner of items in this order item
    ///
    /// - If None: belongs to order.receiver_id (default)
    /// - If Some(user_id): will be transferred to this user after delivery
    ///
    /// When order is fulfilled:
    /// 1. Instances are created with owner = order.receiver_id
    /// 2. If target_owner_id.is_some(), auto-create UserTransaction
    pub target_owner_id: Option<UserId>,

    /// The created instance (None until fulfilled)
    pub instance_id: Option<ProductInstanceId>,
    /// The timestamp when the instance was created
    pub fulfilled_at: Option<DateTime<Utc>>,

    /// The transaction that will transfer this item to target owner
    /// None if: 1) target_owner_id is None (belongs to receiver)
    ///       or 2) target_owner == receiver (no transfer needed)
    ///       or 3) not yet fulfilled
    pub transfer_transaction_id: Option<UserTransactionId>,
}
