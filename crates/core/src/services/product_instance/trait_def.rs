use super::*;
use crate::models::product::ProductInstance;

/// Service for managing product instances (Port).
///
/// This service handles operations related to individual items owned by users:
/// - Retrieving product instances
/// - Listing user's instances
/// - Updating instance status
pub trait ProductInstanceService: Send + Sync + 'static {
    /// Get a specific product instance (item owned by a user).
    fn get_product_instance(
        &self,
        req: GetProductInstanceRequest,
    ) -> impl Future<Output = Result<ProductInstance, GetProductInstanceError>> + Send;

    /// List product instances owned by a user.
    fn list_product_instances(
        &self,
        req: ListProductInstancesRequest,
    ) -> impl Future<Output = Result<Vec<ProductInstance>, ListProductInstancesError>> + Send;

    /// Consume a product instance (e.g. use a ticket, eat food).
    fn consume_product_instance(
        &self,
        req: ConsumeProductInstanceRequest,
    ) -> impl Future<Output = Result<ProductInstance, ConsumeProductInstanceError>> + Send;

    /// Mark a product instance as lost (cannot find it).
    fn mark_product_instance_lost(
        &self,
        req: MarkProductInstanceLostRequest,
    ) -> impl Future<Output = Result<ProductInstance, MarkProductInstanceLostError>> + Send;

    /// Mark a product instance as destroyed (broken, thrown away).
    fn mark_product_instance_destroyed(
        &self,
        req: MarkProductInstanceDestroyedRequest,
    ) -> impl Future<Output = Result<ProductInstance, MarkProductInstanceDestroyedError>> + Send;
}
