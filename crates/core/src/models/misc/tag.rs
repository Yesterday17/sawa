use uuid::{NonNilUuid, Uuid};

use super::NonEmptyString;

/// Tag represents a classification or categorization label.
///
/// Tags can be used to categorize products/variants by various dimensions:
/// - Characters (e.g., "Hatsune Miku", "Kagamine Rin")
/// - Series (e.g., "VOCALOID", "Project SEKAI")
/// - Brands (e.g., "Good Smile Company")
/// - Themes (e.g., "2024 Birthday", "Christmas")
/// - Events (e.g., "Comiket 103")
///
/// Tags are independent aggregates that can be associated with ProductVariants
/// via many-to-many relationships.
pub struct Tag {
    /// The unique identifier for the tag.
    pub id: TagId,

    /// The name of the tag.
    pub name: NonEmptyString,

    /// Optional description of the tag.
    pub description: String,

    /// Parent tag for hierarchical organization (optional).
    ///
    /// This can be used to represent relationships like:
    /// - Character → Series (e.g., "Hatsune Miku" → "VOCALOID")
    /// - Series → Franchise
    ///
    /// Note: This creates a simple tree structure. Cycles should be prevented
    /// at the application layer.
    pub parent_tag_id: Option<TagId>,

    /// Display order for sorting tags.
    pub sort_order: i32,
}

impl Tag {
    /// Create a new tag without parent.
    pub fn new(name: NonEmptyString) -> Self {
        Self {
            id: TagId::new(),
            name,
            description: String::new(),
            parent_tag_id: None,
            sort_order: 0,
        }
    }

    /// Create a new tag with parent.
    pub fn with_parent(name: NonEmptyString, parent_id: TagId) -> Self {
        Self {
            id: TagId::new(),
            name,
            description: String::new(),
            parent_tag_id: Some(parent_id),
            sort_order: 0,
        }
    }

    /// Set the description of this tag.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Set the parent tag.
    pub fn set_parent(&mut self, parent_id: Option<TagId>) {
        self.parent_tag_id = parent_id;
    }

    /// Set the sort order.
    pub fn set_sort_order(&mut self, sort_order: i32) {
        self.sort_order = sort_order;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId(pub NonNilUuid);

impl TagId {
    pub fn new() -> Self {
        Self(NonNilUuid::new(Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}
