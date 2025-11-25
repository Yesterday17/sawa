use crate::models::{
    misc::{MediaId, NonEmptyString, Price, TagId},
    product::{MysteryBoxConfig, ProductId, ProductVariantId},
};

/// Request to get a product by ID.
pub struct GetProductRequest {
    pub id: ProductId,
}

/// Request to create a new product.
pub struct CreateProductRequest {
    pub name: NonEmptyString,
    pub description: String,
    pub medias: Vec<MediaId>,
}

/// Request to list products.
pub struct ListProductsRequest {}

/// Request to get a product variant by ID.
pub struct GetProductVariantRequest {
    pub id: ProductVariantId,
}

/// Request to load multiple product variants by their IDs.
pub struct LoadProductVariantsRequest {
    /// The IDs of the product variants to load.
    pub ids: Vec<ProductVariantId>,
}

/// Request to create a new product variant.
pub struct CreateProductVariantRequest {
    pub product_id: ProductId,
    pub name: NonEmptyString,
    pub description: String,
    pub medias: Vec<MediaId>,
    pub tags: Vec<NonEmptyString>,
    pub price: Option<Price>,
    pub mystery_box: Option<MysteryBoxConfig>,
    pub sort_order: i32,
}

/// Request to list product variants.
pub struct ListProductVariantsRequest {
    /// Filter by product ID.
    pub product_id: Option<ProductId>,
    /// Filter by tags.
    pub tags: Option<Vec<TagId>>,
    /// The tag match policy.
    pub tag_match_policy: TagMatchPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum TagMatchPolicy {
    /// Match any of the specified tags.
    Any,
    /// Match all of the specified tags.
    All,
}
