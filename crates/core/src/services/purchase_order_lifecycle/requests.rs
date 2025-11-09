use crate::models::purchase::PurchaseOrderId;

/// Request to fulfill a purchase order.
pub struct FulfillOrderRequest {
    pub order_id: PurchaseOrderId,
}

/// Request to cancel a purchase order.
pub struct CancelOrderRequest {
    pub order_id: PurchaseOrderId,
    pub reason: Option<String>,
}

