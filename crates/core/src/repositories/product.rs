use std::future::Future;

use crate::{
    error::RepositoryError,
    models::product::{Product, ProductId, ProductVariant, ProductVariantId},
};

/// Repository for the Product aggregate.
///
/// Product is a simple aggregate containing product-level information.
/// ProductVariants are managed separately through ProductVariantRepository.
pub trait ProductRepository: Send + Sync + 'static {
    /// Find a product by its ID.
    fn find_by_id(
        &self,
        id: &ProductId,
    ) -> impl Future<Output = Result<Option<Product>, RepositoryError>> + Send;

    /// Find all products.
    ///
    /// Note: In production, consider adding pagination.
    fn find_all(&self) -> impl Future<Output = Result<Vec<Product>, RepositoryError>> + Send;

    /// Save a product (create or update).
    fn save(&self, product: &Product) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete a product by its ID.
    ///
    /// Note: This should fail if there are any variants referencing this product.
    /// Variants should be deleted first through ProductVariantRepository.
    fn delete(&self, id: &ProductId) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}

/// Repository for ProductVariant aggregate.
///
/// ProductVariant is an independent aggregate root that can be modified
/// without loading the entire Product.
pub trait ProductVariantRepository: Send + Sync + 'static {
    /// Find a variant by its ID.
    fn find_by_id(
        &self,
        id: &ProductVariantId,
    ) -> impl Future<Output = Result<Option<ProductVariant>, RepositoryError>> + Send;

    /// Find all variants for a specific product.
    fn find_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> impl Future<Output = Result<Vec<ProductVariant>, RepositoryError>> + Send;

    /// Find all variants.
    ///
    /// Note: In production, consider adding pagination.
    fn find_all(&self)
    -> impl Future<Output = Result<Vec<ProductVariant>, RepositoryError>> + Send;

    /// Save a variant (create or update).
    fn save(
        &self,
        variant: &ProductVariant,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete a variant by its ID.
    ///
    /// Note: This should fail if there are any ProductInstances or PurchaseOrderItems
    /// referencing this variant.
    fn delete(
        &self,
        id: &ProductVariantId,
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
