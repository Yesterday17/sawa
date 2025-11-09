use url::Url;
use uuid::NonNilUuid;

pub struct Media {
    /// The unique identifier for the media.
    pub id: MediaId,

    /// The publicly accessible URL of the media.
    ///
    /// TODO: This may be changed to a more complex structure in the future to support better authentication and access control.
    pub url: Url,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MediaId(pub NonNilUuid);

impl MediaId {
    pub fn new() -> Self {
        Self(uuid::NonNilUuid::new(uuid::Uuid::now_v7()).expect("UUID v7 should never be nil"))
    }
}
