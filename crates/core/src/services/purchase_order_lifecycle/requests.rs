use crate::models::{purchase::PurchaseOrderId, user::UserId};

/// Request to fulfill a purchase order.
pub struct FulfillOrderRequest {
    /// The user performing this operation.
    /// Must be the creator or receiver of the order.
    pub user_id: UserId,

    /// The ID of the order to fulfill.
    pub order_id: PurchaseOrderId,
}

/// Request to cancel a purchase order.
pub struct CancelOrderRequest {
    /// The user performing this operation.
    /// Must be the creator or receiver of the order.
    pub user_id: UserId,

    /// The ID of the order to cancel.
    pub order_id: PurchaseOrderId,

    /// Optional reason for cancellation.
    pub reason: Option<String>,
}
