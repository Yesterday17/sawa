use crate::models::misc::MediaId;
use url::Url;

/// Request to get a media item by ID.
pub struct GetMediaRequest {
    pub id: MediaId,
}

/// Request to create a new media entry.
pub struct CreateMediaRequest {
    pub url: Url,
}
