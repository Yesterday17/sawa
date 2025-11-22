use sawa_core::{
    models::product::{Product, ProductVariant},
    repositories::*,
    services::{
        CreateProductError, CreateProductVariantError, GetProductError, GetProductVariantError,
        ListProductVariantsError, ListProductsError, ProductService,
    },
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> ProductService for Service<P, PV, PI, PO, UT, U, T, M>
where
    P: ProductRepository,
    PV: ProductVariantRepository,
    PI: ProductInstanceRepository,
    PO: PurchaseOrderRepository,
    UT: UserTransactionRepository,
    U: UserRepository,
    T: TagRepository,
    M: MediaRepository,
{
    async fn get_product(
        &self,
        req: sawa_core::services::GetProductRequest,
    ) -> Result<Product, GetProductError> {
        self.product
            .find_by_id(&req.id)
            .await?
            .ok_or(GetProductError::NotFound)
    }

    async fn create_product(
        &self,
        req: sawa_core::services::CreateProductRequest,
    ) -> Result<Product, CreateProductError> {
        let mut product = Product::new(req.name, req.description);
        for media in req.medias {
            product.add_media(media);
        }

        self.product.save(&product).await?;

        Ok(product)
    }

    async fn list_products(
        &self,
        _req: sawa_core::services::ListProductsRequest,
    ) -> Result<Vec<Product>, ListProductsError> {
        // Currently no filtering in request
        Ok(self.product.find_all().await?)
    }

    async fn get_product_variant(
        &self,
        req: sawa_core::services::GetProductVariantRequest,
    ) -> Result<ProductVariant, GetProductVariantError> {
        self.product_variant
            .find_by_id(&req.id)
            .await?
            .ok_or(GetProductVariantError::NotFound)
    }

    async fn create_product_variant(
        &self,
        req: sawa_core::services::CreateProductVariantRequest,
    ) -> Result<ProductVariant, CreateProductVariantError> {
        // 1. Check if product exists
        if self.product.find_by_id(&req.product_id).await?.is_none() {
            return Err(CreateProductVariantError::ProductNotFound);
        }

        // 2. Create variant
        let mut variant = if let Some(mb_config) = req.mystery_box {
            ProductVariant::mystery_box(
                req.product_id,
                req.name,
                mb_config.items_count,
                mb_config.possible_variants,
            )
        } else {
            ProductVariant::new(req.product_id, req.name)
        };

        variant.set_description(req.description);
        if let Some(price) = req.price {
            variant.set_price(price);
        }
        variant.set_sort_order(req.sort_order);

        for media in req.medias {
            variant.add_media(media);
        }

        for tag in req.tags {
            variant.add_tag(tag);
        }

        // 3. Save
        self.product_variant.save(&variant).await?;

        Ok(variant)
    }

    async fn list_product_variants(
        &self,
        req: sawa_core::services::ListProductVariantsRequest,
    ) -> Result<Vec<ProductVariant>, ListProductVariantsError> {
        let variants = if let Some(product_id) = req.product_id {
            // Filter by product ID
            let mut variants = self.product_variant.find_by_product_id(&product_id).await?;

            // If tags are also provided, filter in memory
            if let Some(tags) = req.tags {
                if !tags.is_empty() {
                    variants.retain(|v| {
                        if req.match_all_tags {
                            tags.iter().all(|t| v.has_tag(t))
                        } else {
                            tags.iter().any(|t| v.has_tag(t))
                        }
                    });
                }
            }
            variants
        } else if let Some(tags) = req.tags {
            // Filter by tags only
            if tags.is_empty() {
                self.product_variant.find_all().await?
            } else if req.match_all_tags {
                self.product_variant.find_by_tags_all(&tags).await?
            } else {
                self.product_variant.find_by_tags_any(&tags).await?
            }
        } else {
            // No filters
            self.product_variant.find_all().await?
        };

        Ok(variants)
    }
}
