use std::num::NonZeroU32;

use crate::models::{
    misc::Price,
    product::ProductVariantId,
    purchase::{PurchaseOrderId, PurchaseOrderItemId},
    user::UserId,
};

/// Request to create a new purchase order.
pub struct CreateOrderRequest {
    pub creator_id: UserId,
    pub receiver_id: UserId,
}

/// Request to add an item to an order.
pub struct AddOrderItemRequest {
    pub order_id: PurchaseOrderId,
    pub variant_id: ProductVariantId,
    pub quantity: NonZeroU32,
    pub unit_price: Option<Price>,
}

/// Request to submit mystery box results.
pub struct SubmitMysteryBoxResultsRequest {
    pub order_id: PurchaseOrderId,
    pub order_item_id: PurchaseOrderItemId,
    pub received_variants: Vec<ProductVariantId>,
}

/// Request to get an order by ID.
pub struct GetOrderRequest {
    pub order_id: PurchaseOrderId,
}
