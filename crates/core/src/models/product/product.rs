use uuid::{NonNilUuid, Uuid};

use crate::models::misc::{MediaId, NonEmptyString};

/// Product represents a product in the catalog.
///
/// Product is a simple aggregate root that contains product-level information.
/// ProductVariants are separate aggregates that reference the Product via product_id.
///
/// This design allows efficient independent modification of variants without
/// loading the entire product hierarchy.
#[derive(Debug, Clone)]
pub struct Product {
    /// The unique identifier for the product.
    pub id: ProductId,

    /// Name of the product.
    ///
    /// We may add localization support later, but for now a simple string should be enough.
    /// Users can just write the name in multiple languages if they want.
    pub name: NonEmptyString,

    /// Description of the product.
    ///
    /// If there is no description, just leave it as an empty string.
    pub description: String,

    /// A product may have multiple associated media items (e.g., images, videos).
    pub medias: Vec<MediaId>,
}

impl Product {
    /// Create a new product.
    pub fn new(name: NonEmptyString, description: String) -> Self {
        Self {
            id: ProductId::new(),
            name,
            description,
            medias: Vec::new(),
        }
    }

    /// Add a media to this product.
    pub fn add_media(&mut self, media: MediaId) {
        self.medias.push(media);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductId(pub NonNilUuid);

impl ProductId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}
