use uuid::NonNilUuid;

use crate::models::misc::{MediaId, NonEmptyString};

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
    pub description: NonEmptyString,

    /// A product may have multiple associated media items (e.g., images, videos).
    pub medias: Vec<MediaId>,
}

pub struct ProductId(pub NonNilUuid);
