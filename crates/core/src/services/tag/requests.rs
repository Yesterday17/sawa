use crate::models::misc::{NonEmptyString, TagId};

/// Request to get a tag by ID.
pub struct GetTagRequest {
    pub id: TagId,
}

/// Request to get multiple tags by ID.
pub struct LoadTagsRequest {
    pub ids: Vec<TagId>,
}

/// Request to create a new tag.
pub struct CreateTagRequest {
    pub name: NonEmptyString,
    pub description: String,
    pub parent_id: Option<TagId>,
}
