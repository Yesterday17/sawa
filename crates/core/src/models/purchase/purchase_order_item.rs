use std::num::NonZeroU32;

use chrono::{DateTime, Utc};
use uuid::{NonNilUuid, Uuid};

use crate::models::{
    misc::Price,
    product::{
        MysteryBoxConfig, ProductInstance, ProductInstanceId, ProductInstanceStatus,
        ProductInstanceStatusHistory, ProductVariantId,
    },
    transfer::{OwnershipTransfer, TransferReason, UserTransactionId},
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
    pub line_items: Vec<PurchaseOrderLineItem>,

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

/// A variant pending instance creation, with ownership assignment
#[derive(Debug, Clone)]
pub struct PurchaseOrderLineItem {
    id: PurchaseOrderLineItemId,

    /// The purchase order item this order item belongs to
    purchase_order_item_id: PurchaseOrderItemId,

    /// The variant to create instance for
    variant_id: ProductVariantId,

    /// The ultimate owner of items in this order item
    ///
    /// - If None: belongs to order.receiver_id (default)
    /// - If Some(user_id): will be transferred to this user after delivery
    ///
    /// When order is fulfilled:
    /// 1. Instances are created with owner = order.receiver_id
    /// 2. If target_owner_id.is_some(), auto-create UserTransaction
    target_owner_id: Option<UserId>,

    /// The created instance (None until fulfilled)
    instance_id: Option<ProductInstanceId>,
    /// The timestamp when the instance was created
    fulfilled_at: Option<DateTime<Utc>>,

    /// The transaction that will transfer this item to target owner
    /// None if: 1) target_owner == receiver (no transfer needed)
    ///       or 2) not yet fulfilled
    transfer_transaction_id: Option<UserTransactionId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurchaseOrderLineItemId(pub NonNilUuid);

impl PurchaseOrderLineItemId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

impl PurchaseOrderLineItem {
    pub fn new(
        variant_id: ProductVariantId,
        purchase_order_item_id: PurchaseOrderItemId,
        target_owner_id: Option<UserId>,
    ) -> Self {
        Self {
            id: PurchaseOrderLineItemId::new(),
            variant_id,
            purchase_order_item_id,
            target_owner_id,
            instance_id: None,
            fulfilled_at: None,
            transfer_transaction_id: None,
        }
    }

    pub fn is_fulfilled(&self) -> bool {
        self.instance_id.is_some() && self.fulfilled_at.is_some()
    }

    pub fn to_product_instance(&self, receiver_id: UserId) -> ProductInstance {
        let now = Utc::now();
        ProductInstance {
            id: ProductInstanceId::new(),
            variant_id: self.variant_id,
            owner_id: receiver_id,
            status: ProductInstanceStatus::Active,
            source_order_line_item_id: self.id,
            created_at: now,
            transfer_history: vec![OwnershipTransfer {
                from_user_id: None,
                to_user_id: receiver_id,
                reason: TransferReason::Purchase,
                transferred_at: now,
            }],
            status_history: vec![ProductInstanceStatusHistory {
                status: ProductInstanceStatus::Active,
                changed_at: now,
                reason: None,
            }],
        }
    }

    pub fn fulfill(&mut self, instance: &ProductInstance) {
        self.instance_id = Some(instance.id);
        self.fulfilled_at = Some(instance.created_at);
    }

    pub fn transaction_product_instance_id(
        &self,
        receiver_id: UserId,
        order_owner_id: UserId,
    ) -> Option<ProductInstanceId> {
        if let Some(instance_id) = self.instance_id {
            let target_user_id = self.target_owner_id(order_owner_id);
            if receiver_id != target_user_id {
                return Some(instance_id);
            }
        }

        None
    }

    // Getters for private fields
    pub fn id(&self) -> &PurchaseOrderLineItemId {
        &self.id
    }

    pub fn purchase_order_item_id(&self) -> PurchaseOrderItemId {
        self.purchase_order_item_id
    }

    pub fn variant_id(&self) -> ProductVariantId {
        self.variant_id
    }

    pub fn target_owner_id(&self, order_owner_id: UserId) -> UserId {
        self.target_owner_id.unwrap_or(order_owner_id)
    }

    pub fn instance_id(&self) -> Option<ProductInstanceId> {
        self.instance_id
    }

    pub fn set_transfer_transaction_id(&mut self, tx_id: UserTransactionId) {
        self.transfer_transaction_id = Some(tx_id);
    }
}
