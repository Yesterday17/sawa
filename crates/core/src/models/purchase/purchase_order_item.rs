use std::num::NonZeroU32;

use uuid::NonNilUuid;

use crate::models::{
    misc::Price,
    product::{MysteryBoxConfig, ProductInstanceId, ProductVariantId},
};

/// A single item in a purchase order.
pub struct PurchaseOrderItem {
    pub id: PurchaseOrderItemId,

    /// Which variant to purchase
    pub product_variant_id: ProductVariantId,

    /// Snapshot of mystery box config at time of purchase
    /// If Some, this item was a mystery box purchase
    /// If None, this was a regular item purchase
    pub mystery_box_snapshot: Option<MysteryBoxConfig>,

    /// How many units to purchase
    pub quantity: NonZeroU32,

    /// Price at time of order (snapshot, immutable)
    pub unit_price: Price,

    /// Results of opening the mystery box (user input)
    ///
    /// Invariants:
    /// - For mystery boxes: length SHOULD equal `quantity * mystery_box_snapshot.count`
    /// - For regular items: SHOULD be empty
    /// - Each variant SHOULD be in `mystery_box_snapshot.possible_variants` (soft requirement)
    ///
    /// Note: Validation is lenient to allow for:
    /// - Special promotions/bonuses
    /// - User errors that can be corrected later
    pub received_items: Vec<ProductVariantId>,

    /// Product instances created for this order item
    /// Empty until order is fulfilled
    pub created_instances: Vec<ProductInstanceId>,

    /// Item status
    pub status: PurchaseOrderItemStatus,
}

pub struct PurchaseOrderItemId(pub NonNilUuid);

/// The status of a purchase order item.
#[derive(Debug, Clone, Copy)]
pub enum PurchaseOrderItemStatus {
    /// Mystery box waiting for user to fill in what they received
    /// Regular items skip this status
    AwaitingInput,

    /// All information complete, ready to create instances
    /// This is the "gate" status - order can only be fulfilled when ALL items are Pending
    Pending,

    /// Instances created successfully
    Fulfilled,

    /// Cancelled
    Cancelled,
}
