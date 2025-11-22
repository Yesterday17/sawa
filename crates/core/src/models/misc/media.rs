use url::Url;

crate::create_entity_id!(MediaId);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Media {
    /// The unique identifier for the media.
    pub id: MediaId,

    /// The publicly accessible URL of the media.
    ///
    /// TODO: This may be changed to a more complex structure in the future to support better authentication and access control.
    pub url: Url,
}
