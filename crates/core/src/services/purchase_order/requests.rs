use std::num::NonZeroU32;

use crate::models::{
    misc::{Address, Price},
    product::ProductVariantId,
    purchase::{OrderRoleFilter, PurchaseOrderId, PurchaseOrderItemId, PurchaseOrderStatus},
    user::UserId,
};

/// Request to create a new purchase order.
pub struct CreateOrderRequest {
    /// The user creating the order (will be set as creator_id).
    pub user_id: UserId,

    /// The user who will receive the shipment (default to creator).
    /// Can be different for group buy scenarios.
    pub receiver_id: Option<UserId>,

    /// Shipping/delivery address (optional, for physical goods).
    /// If None, the order is assumed to be for digital goods.
    pub shipping_address: Option<Address>,

    /// The currency of the order.
    pub total_price: Option<Price>,

    /// Initial items to add to the order.
    pub items: Vec<CreateOrderItemRequest>,
}

/// Request to add an item to an order (during creation).
pub struct CreateOrderItemRequest {
    /// The variant to purchase.
    pub variant_id: ProductVariantId,

    /// The user who will own the instance created for this line item.
    /// If None, defaults to the order receiver.
    pub owner_id: Option<UserId>,

    /// Quantity to purchase.
    pub quantity: NonZeroU32,

    /// Price at time of order.
    pub unit_price: Option<Price>,
}

/// Request to add an item to an order.
pub struct AddOrderItemRequest {
    /// The user performing this operation.
    pub user_id: UserId,

    /// The order to add item to.
    pub order_id: PurchaseOrderId,

    /// The variant to purchase.
    pub variant_id: ProductVariantId,

    /// The user who will own the instance created for this line item.
    pub owner_id: UserId,

    /// Quantity to purchase.
    pub quantity: NonZeroU32,

    /// Price at time of order.
    pub unit_price: Option<Price>,
}

/// Request to submit mystery box results.
pub struct SubmitMysteryBoxResultsRequest {
    /// The user performing this operation.
    pub user_id: UserId,

    /// The order containing the mystery box.
    pub order_id: PurchaseOrderId,

    /// The specific order item (mystery box).
    pub order_item_id: PurchaseOrderItemId,

    /// The user who will own the instance created for this line item.
    pub owner_id: UserId,

    /// What the user received from opening the box.
    pub received_variants: Vec<ProductVariantId>,
}

/// Request to get an order by ID.
pub struct GetOrderRequest {
    /// The user requesting the order.
    pub user_id: UserId,

    /// The order to retrieve.
    pub order_id: PurchaseOrderId,
}

/// Request to list orders for a user.
pub struct ListOrdersRequest {
    /// The user requesting the orders.
    pub user_id: UserId,

    /// Filter by user role in the order.
    pub role: OrderRoleFilter,

    /// Filter by order status.
    pub status: Option<PurchaseOrderStatus>,
}
