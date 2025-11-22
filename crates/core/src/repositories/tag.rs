use crate::{
    errors::RepositoryError,
    models::misc::{Tag, TagId},
};

/// Repository for the Tag aggregate.
///
/// Tags are used for categorizing ProductVariants by various dimensions
/// such as characters, series, brands, themes, events, etc.
pub trait TagRepository: Send + Sync + 'static {
    /// Find a tag by its ID.
    fn find_by_id(
        &self,
        id: &TagId,
    ) -> impl Future<Output = Result<Option<Tag>, RepositoryError>> + Send;

    /// Find all tags.
    ///
    /// Note: In production, consider adding pagination or filtering.
    fn find_all(&self) -> impl Future<Output = Result<Vec<Tag>, RepositoryError>> + Send;

    /// Find a tag by its name.
    fn find_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<Tag>, RepositoryError>> + Send;

    /// Find tags by name (case-insensitive search).
    ///
    /// This is useful for tag autocomplete or search functionality.
    fn find_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<Tag>, RepositoryError>> + Send;

    /// Find all child tags of a parent tag.
    ///
    /// This is useful for hierarchical tag navigation.
    fn find_by_parent(
        &self,
        parent_id: &TagId,
    ) -> impl Future<Output = Result<Vec<Tag>, RepositoryError>> + Send;

    /// Find all root tags (tags without parent).
    fn find_roots(&self) -> impl Future<Output = Result<Vec<Tag>, RepositoryError>> + Send;

    /// Save a tag (create or update).
    fn save(&self, tag: &Tag) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete a tag by its ID.
    ///
    /// Note: This should fail if there are ProductVariants still using this tag.
    fn delete(&self, id: &TagId) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
