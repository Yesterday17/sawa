use super::*;
use crate::models::misc::Media;

/// Service for managing media (Port).
///
/// This service handles media operations:
/// - Creating new media entries
/// - Retrieving media information
pub trait MediaService: Send + Sync + 'static {
    /// Get a media item by its ID.
    fn get_media(
        &self,
        req: GetMediaRequest,
    ) -> impl Future<Output = Result<Media, GetMediaError>> + Send;

    /// Create a new media entry.
    fn create_media(
        &self,
        req: CreateMediaRequest,
    ) -> impl Future<Output = Result<Media, CreateMediaError>> + Send;
}
