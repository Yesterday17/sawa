use crate::{
    errors::RepositoryError,
    models::{
        purchase::{OrderRoleFilter, PurchaseOrder, PurchaseOrderId, PurchaseOrderStatus},
        user::UserId,
    },
};

/// Repository for the PurchaseOrder aggregate.
///
/// PurchaseOrder is the aggregate root that contains PurchaseOrderItems.
/// Items should only be accessed through the PurchaseOrder aggregate.
pub trait PurchaseOrderRepository: Send + Sync + 'static {
    /// Find an order by its ID and verify permission based on user ID.
    fn find_by_id(
        &self,
        id: &PurchaseOrderId,
        user_id: &UserId,
    ) -> impl Future<Output = Result<Option<PurchaseOrder>, RepositoryError>> + Send;

    /// Find all orders for a specific user.
    ///
    /// If status is Some, only returns orders with that status.
    /// If status is None, returns all orders.
    fn find_by_user(
        &self,
        user_id: &UserId,
        role: OrderRoleFilter,
        status: Option<PurchaseOrderStatus>,
    ) -> impl Future<Output = Result<Vec<PurchaseOrder>, RepositoryError>> + Send;

    /// Save an order (create or update).
    ///
    /// This should save the entire aggregate including all items.
    fn save(
        &self,
        order: &PurchaseOrder,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete an order by its ID.
    ///
    /// Note: Consider soft deletion or status change instead.
    fn delete(
        &self,
        id: &PurchaseOrderId,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
