use crate::{
    errors::RepositoryError,
    models::{
        product::{ProductInstance, ProductInstanceId, ProductInstanceStatus, ProductVariantId},
        user::UserId,
    },
};

/// Repository for the ProductInstance aggregate.
///
/// ProductInstance represents individual items owned by users.
pub trait ProductInstanceRepository: Send + Sync + 'static {
    /// Find an instance by its ID.
    fn find_by_id(
        &self,
        id: &ProductInstanceId,
    ) -> impl Future<Output = Result<Option<ProductInstance>, RepositoryError>> + Send;

    /// Find all instances owned by a user.
    fn find_by_owner(
        &self,
        owner_id: &UserId,
    ) -> impl Future<Output = Result<Vec<ProductInstance>, RepositoryError>> + Send;

    /// Find all instances of a specific variant owned by a user.
    fn find_by_owner_and_variant(
        &self,
        owner_id: &UserId,
        variant_id: &ProductVariantId,
    ) -> impl Future<Output = Result<Vec<ProductInstance>, RepositoryError>> + Send;

    /// Find all instances owned by a user with a specific status.
    fn find_by_owner_and_status(
        &self,
        owner_id: &UserId,
        status: ProductInstanceStatus,
    ) -> impl Future<Output = Result<Vec<ProductInstance>, RepositoryError>> + Send;

    /// Save an instance (create or update).
    fn save(
        &self,
        instance: &ProductInstance,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Save multiple instances atomically.
    ///
    /// This is useful when creating instances from a fulfilled order.
    fn save_batch(
        &self,
        instances: &[ProductInstance],
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete an instance by its ID.
    fn delete(
        &self,
        id: &ProductInstanceId,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
