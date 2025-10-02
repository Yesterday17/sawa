use std::num::NonZeroU32;

use uuid::NonNilUuid;

use crate::models::{
    misc::{MediaId, NonEmptyString, Price},
    product::ProductId,
};

/// Represents a specific variant of a product.
///
/// For example, if "T-Shirt" is a `Product`, then "Red, Size M" could be a `ProductVariant`.
/// This allows for different versions of the same product to be represented with their own unique attributes, such as price and name.
///
/// For gacha goods, a special variant
pub struct ProductVariant {
    /// The unique identifier for the product variant.
    pub id: ProductVariantId,

    /// The ID of the product to which this variant belongs.
    pub product_id: ProductId,

    /// The name of the product variant.
    pub name: NonEmptyString,

    /// The media items associated with the product variant.
    pub medias: Vec<MediaId>,

    /// The price of the product variant.
    pub price: Price,

    /// The mystery box configuration of the product variant.
    ///
    /// If present, this variant is a mystery box.
    pub mystery_box: Option<MysteryBoxConfig>,

    /// The sort order of the product variant among other variants of the same product.
    /// Variants with lower order values should be displayed before those with higher values.
    ///
    /// Variants with the same order value would be displayed in the order of variant_id.
    pub sort_order: i32,
}

#[derive(Debug, Clone)]
pub struct ProductVariantId(pub NonNilUuid);

#[derive(Debug, Clone)]
pub struct MysteryBoxConfig {
    /// Number of items to give per mystery box
    pub count: NonZeroU32,

    /// Possible prizes
    pub possible_variants: Vec<ProductVariantId>,
}
