use super::*;
use crate::models::product::{Product, ProductVariant};

/// Service for managing products and variants (Port).
///
/// This service handles the core catalog management operations:
/// - Creating and retrieving products
/// - Managing product variants (SKUs)
pub trait ProductService: Send + Sync + 'static {
    /// Get a product by its ID.
    fn get_product(
        &self,
        req: GetProductRequest,
    ) -> impl Future<Output = Result<Product, GetProductError>> + Send;

    /// Create a new product.
    fn create_product(
        &self,
        req: CreateProductRequest,
    ) -> impl Future<Output = Result<Product, CreateProductError>> + Send;

    /// List products.
    fn list_products(
        &self,
        req: ListProductsRequest,
    ) -> impl Future<Output = Result<Vec<Product>, ListProductsError>> + Send;

    /// Get a product variant by its ID.
    fn get_product_variant(
        &self,
        req: GetProductVariantRequest,
    ) -> impl Future<Output = Result<ProductVariant, GetProductVariantError>> + Send;

    /// Create a new product variant.
    fn create_product_variant(
        &self,
        req: CreateProductVariantRequest,
    ) -> impl Future<Output = Result<ProductVariant, CreateProductVariantError>> + Send;

    /// List product variants.
    fn list_product_variants(
        &self,
        req: ListProductVariantsRequest,
    ) -> impl Future<Output = Result<Vec<ProductVariant>, ListProductVariantsError>> + Send;
}
