use crate::models::{
    product::{
        ProductInstance, ProductInstanceId, ProductInstanceStatus, ProductInstanceStatusHistory,
        ProductInstanceStatusHistoryId, ProductVariantId,
    },
    purchase::PurchaseOrderItemId,
    transfer::{ProductInstanceTransferHistory, ProductInstanceTransferHistoryId, TransferReason},
    user::UserId,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

crate::create_entity_id!(PurchaseOrderLineItemId);

/// A variant pending instance creation, with ownership assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PurchaseOrderLineItem {
    pub id: PurchaseOrderLineItemId,

    /// The purchase order item this order item belongs to
    pub purchase_order_item_id: PurchaseOrderItemId,

    /// The variant to create instance for
    pub variant_id: ProductVariantId,

    /// The ultimate owner of instance created for this line item
    pub owner_id: UserId,

    /// The created instance (None until fulfilled)
    pub instance_id: Option<ProductInstanceId>,
    /// The timestamp when the instance was created
    pub fulfilled_at: Option<DateTime<Utc>>,
}

impl PurchaseOrderLineItem {
    pub fn new(
        variant_id: ProductVariantId,
        purchase_order_item_id: PurchaseOrderItemId,
        owner_id: UserId,
    ) -> Self {
        Self {
            id: PurchaseOrderLineItemId::new(),
            variant_id,
            purchase_order_item_id,
            owner_id,
            instance_id: None,
            fulfilled_at: None,
        }
    }

    pub fn is_fulfilled(&self) -> bool {
        self.instance_id.is_some() && self.fulfilled_at.is_some()
    }

    pub fn to_product_instance(&self, holder_id: UserId) -> ProductInstance {
        let now = Utc::now();
        ProductInstance {
            id: ProductInstanceId::new(),
            variant_id: self.variant_id,
            owner_id: self.owner_id,
            holder_id,
            status: ProductInstanceStatus::Active,
            source_order_line_item_id: self.id,
            created_at: now,
            transfer_history: vec![ProductInstanceTransferHistory {
                id: ProductInstanceTransferHistoryId::new(),
                from_owner_id: None,
                from_holder_id: None,
                to_owner_id: self.owner_id,
                to_holder_id: holder_id,
                reason: TransferReason::Purchase,
                transferred_at: now,
            }],
            status_history: vec![ProductInstanceStatusHistory {
                id: ProductInstanceStatusHistoryId::new(),
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

    pub fn instance_id(&self) -> Option<ProductInstanceId> {
        self.instance_id
    }
}
