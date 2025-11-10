use std::num::NonZeroU32;

use uuid::{NonNilUuid, Uuid};

use crate::models::{misc::Price, product::ProductVariantId, purchase::PurchaseOrderLineItem};

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
    /// - For mystery boxes: length SHOULD equal `quantity * mystery_box.count`
    ///   - Each variant SHOULD be in `mystery_box.possible_variants` (soft requirement)
    ///
    /// Note: Validation is lenient to allow for:
    /// - Special promotions/bonuses
    /// - User errors that can be corrected later
    pub line_items: Vec<PurchaseOrderLineItem>,

    /// Item status
    pub status: PurchaseOrderItemStatus,

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
    /// Waiting for user to fill in `line_items`.
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
