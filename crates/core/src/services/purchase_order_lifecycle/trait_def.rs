use crate::models::purchase::PurchaseOrder;

use super::{CancelOrderError, CancelOrderRequest, FulfillOrderError, FulfillOrderRequest};

/// Service for managing purchase order lifecycle and state transitions (Port).
///
///
/// This service handles order state transitions:
/// - Fulfillment: Creating [ProductInstance]s from [PurchaseOrderLineItem]s
/// - Cancellation: Cancelling incomplete orders (user input errors, etc.)
///
/// # State Transitions
///
/// ```text
/// Incomplete --fulfill--> Completed
/// Incomplete --cancel--> Cancelled
/// ```
///
/// Note: Completed orders CANNOT be cancelled. For returns/refunds of completed
/// orders, a separate refund flow should be implemented in the future.
///
/// Responsibilities:
/// - State validation before transitions
/// - Creating ProductInstances during fulfillment
/// - Creating UserTransactions for group buy scenarios
/// - Updating order and item statuses
pub trait PurchaseOrderLifecycleService: Send + Sync + 'static {
    /// Fulfill a purchase order.
    ///
    /// Transitions order from Incomplete to Completed.
    /// Creates ProductInstances and UserTransactions as needed.
    ///
    /// # Preconditions
    ///
    /// - Order status must be Incomplete
    /// - All order items must be in Pending status
    fn fulfill_order(
        &self,
        req: &FulfillOrderRequest,
    ) -> impl Future<Output = Result<PurchaseOrder, FulfillOrderError>> + Send;

    /// Cancel an incomplete purchase order.
    ///
    /// This operation is ONLY allowed for Incomplete orders.
    /// Completed orders cannot be cancelled (use refund flow instead).
    ///
    /// # Preconditions
    ///
    /// - Order status must be Incomplete
    ///
    /// # Use Cases
    ///
    /// - User entered wrong information and wants to delete the record
    /// - External purchase was cancelled before delivery
    ///
    /// # Errors
    ///
    /// Returns `OrderAlreadyCompleted` if attempting to cancel a completed order.
    fn cancel_order(
        &self,
        req: &CancelOrderRequest,
    ) -> impl Future<Output = Result<PurchaseOrder, CancelOrderError>> + Send;
}
