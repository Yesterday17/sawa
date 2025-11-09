use crate::models::purchase::{PurchaseOrder, PurchaseOrderItemId};

use super::{
    AddOrderItemError, AddOrderItemRequest, CreateOrderError, CreateOrderRequest, GetOrderError,
    GetOrderRequest, SubmitMysteryBoxResultsError, SubmitMysteryBoxResultsRequest,
};

/// Service for managing purchase orders (Port).
///
/// This service handles operations related to purchase order lifecycle:
/// - Creating new orders
/// - Adding items to orders
/// - Submitting mystery box results
/// - Querying orders
pub trait PurchaseOrderService: Send + Sync + 'static {
    /// Create a new purchase order.
    fn create_order(
        &self,
        req: &CreateOrderRequest,
    ) -> impl Future<Output = Result<PurchaseOrder, CreateOrderError>> + Send;

    /// Add an item to an order.
    fn add_order_item(
        &self,
        req: &AddOrderItemRequest,
    ) -> impl Future<Output = Result<PurchaseOrderItemId, AddOrderItemError>> + Send;

    /// Submit mystery box results.
    fn submit_mystery_box_results(
        &self,
        req: &SubmitMysteryBoxResultsRequest,
    ) -> impl Future<Output = Result<(), SubmitMysteryBoxResultsError>> + Send;

    /// Get an order by ID.
    fn get_order(
        &self,
        req: &GetOrderRequest,
    ) -> impl Future<Output = Result<PurchaseOrder, GetOrderError>> + Send;
}
