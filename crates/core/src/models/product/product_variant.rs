use std::num::NonZeroU32;

use uuid::{NonNilUuid, Uuid};

use crate::models::{
    misc::{MediaId, NonEmptyString, Price, TagId},
    product::ProductId,
};

/// Represents a specific variant of a product.
///
/// ProductVariant is an independent aggregate root that references a Product.
///
/// For example, if "T-Shirt" is a `Product`, then "Red, Size M" could be a `ProductVariant`.
/// This allows for different versions of the same product to be represented with their own
/// unique attributes, such as price and name.
///
/// Each variant can be modified independently without loading the entire Product,
/// enabling efficient concurrent updates and better performance.
pub struct ProductVariant {
    /// The unique identifier for the product variant.
    pub id: ProductVariantId,

    /// The ID of the product to which this variant belongs.
    pub product_id: ProductId,

    /// The name of the product variant.
    pub name: NonEmptyString,

    /// The description of the product variant.
    pub description: String,

    /// The media items associated with the product variant.
    pub medias: Vec<MediaId>,

    /// Tags associated with this variant (characters, series, themes, etc.).
    ///
    /// This enables flexible categorization and filtering.
    /// Examples: character names, series names, brands, event names, etc.
    pub tags: Vec<TagId>,

    /// The price of the product variant.
    ///
    /// It might be None if:
    /// - The variant only exists in a bundle and does not have a separate price
    /// - The variant is not for sale
    /// - The user just did not set the price
    pub price: Option<Price>,

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductVariantId(pub NonNilUuid);

impl ProductVariantId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}

#[derive(Debug, Clone)]
pub struct MysteryBoxConfig {
    /// Number of items to give per mystery box
    pub items_count: NonZeroU32,

    /// Possible variants that can be received/select from
    pub possible_variants: Vec<ProductVariantId>,
}

impl ProductVariant {
    /// Create a new regular product variant.
    ///
    /// # Arguments
    /// * `product_id` - The ID of the product this variant belongs to
    /// * `name` - The name of this variant
    pub fn new(product_id: ProductId, name: NonEmptyString) -> Self {
        Self {
            id: ProductVariantId::new(),
            product_id,
            name,
            description: String::new(),
            medias: Vec::new(),
            tags: Vec::new(),
            price: None,
            mystery_box: None,
            sort_order: 0,
        }
    }

    /// Create a new mystery box variant.
    ///
    /// # Arguments
    /// * `product_id` - The ID of the product this variant belongs to
    /// * `name` - The name of this mystery box
    /// * `items_count` - Number of items this mystery box gives
    /// * `possible_variants` - Possible variants that can be obtained
    pub fn mystery_box(
        product_id: ProductId,
        name: NonEmptyString,
        items_count: NonZeroU32,
        possible_variants: Vec<ProductVariantId>,
    ) -> Self {
        Self {
            id: ProductVariantId::new(),
            product_id,
            name,
            description: String::new(),
            medias: Vec::new(),
            tags: Vec::new(),
            price: None,
            mystery_box: Some(MysteryBoxConfig {
                items_count,
                possible_variants,
            }),
            sort_order: 0,
        }
    }

    /// Add a media to this variant.
    pub fn add_media(&mut self, media: MediaId) {
        self.medias.push(media);
    }

    /// Set the description of this variant.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Set the price of this variant.
    pub fn set_price(&mut self, price: Price) {
        self.price = Some(price);
    }

    /// Set the sort order of this variant.
    pub fn set_sort_order(&mut self, sort_order: i32) {
        self.sort_order = sort_order;
    }

    /// Add a tag to this variant.
    pub fn add_tag(&mut self, tag_id: TagId) {
        if !self.tags.contains(&tag_id) {
            self.tags.push(tag_id);
        }
    }

    /// Remove a tag from this variant.
    pub fn remove_tag(&mut self, tag_id: &TagId) {
        self.tags.retain(|id| id != tag_id);
    }

    /// Check if this variant has a specific tag.
    pub fn has_tag(&self, tag_id: &TagId) -> bool {
        self.tags.contains(tag_id)
    }
}
