use crate::models::misc::{TagId, NonEmptyString};

/// Request to get a tag by ID.
pub struct GetTagRequest {
    pub id: TagId,
}

/// Request to create a new tag.
pub struct CreateTagRequest {
    pub name: NonEmptyString,
    pub description: String,
    pub parent_id: Option<TagId>,
}
