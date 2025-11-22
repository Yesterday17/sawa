use super::*;
use crate::models::misc::Tag;

/// Service for managing tags (Port).
///
/// This service handles tag operations:
/// - Creating new tags
/// - Retrieving tags
pub trait TagService: Send + Sync + 'static {
    /// Get a tag by its ID.
    fn get_tag(
        &self,
        req: GetTagRequest,
    ) -> impl Future<Output = Result<Tag, GetTagError>> + Send;

    /// Create a new tag.
    fn create_tag(
        &self,
        req: CreateTagRequest,
    ) -> impl Future<Output = Result<Tag, CreateTagError>> + Send;
}
