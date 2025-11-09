use crate::{
    errors::RepositoryError,
    models::misc::{Media, MediaId},
};

/// Repository for the Media aggregate.
///
/// Media represents uploaded files (images, videos, etc.).
pub trait MediaRepository: Send + Sync + 'static {
    /// Find a media item by its ID.
    fn find_by_id(
        &self,
        id: &MediaId,
    ) -> impl Future<Output = Result<Option<Media>, RepositoryError>> + Send;

    /// Find multiple media items by their IDs.
    ///
    /// This is useful for batch loading media for products.
    fn find_by_ids(
        &self,
        ids: &[MediaId],
    ) -> impl Future<Output = Result<Vec<Media>, RepositoryError>> + Send;

    /// Save a media item (create or update).
    fn save(&self, media: &Media) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    /// Delete a media item by its ID.
    ///
    /// Note: Should check if media is still referenced before deletion.
    fn delete(&self, id: &MediaId) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
