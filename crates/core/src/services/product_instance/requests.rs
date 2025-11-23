use crate::models::{
    product::{ProductInstanceId, ProductInstanceStatus, ProductVariantId},
    user::UserId,
};

/// Request to get a product instance by ID.
pub struct GetProductInstanceRequest {
    pub id: ProductInstanceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ListProductInstancesQueryBy {
    Owned,
    Held,
}

/// Request to list product instances.
pub struct ListProductInstancesRequest {
    /// The user ID to query by (either owner or holder depending on `query_by`).
    pub user_id: UserId,
    /// Whether to query by owner_id or holder_id.
    pub query_by: ListProductInstancesQueryBy,
    pub variant_id: Option<ProductVariantId>,
    pub status: Option<ProductInstanceStatus>,
}

/// Request to consume a product instance.
pub struct ConsumeProductInstanceRequest {
    pub id: ProductInstanceId,
    pub user_id: UserId,
}

/// Request to mark a product instance as lost.
pub struct MarkProductInstanceLostRequest {
    pub id: ProductInstanceId,
    pub user_id: UserId,
    pub reason: Option<String>,
}

/// Request to mark a product instance as destroyed.
pub struct MarkProductInstanceDestroyedRequest {
    pub id: ProductInstanceId,
    pub user_id: UserId,
    pub reason: Option<String>,
}
