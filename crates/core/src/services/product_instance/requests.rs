use crate::models::{
    product::{ProductInstanceId, ProductInstanceStatus, ProductVariantId},
    user::UserId,
};

/// Request to get a product instance by ID.
pub struct GetProductInstanceRequest {
    pub id: ProductInstanceId,
}

/// Request to list product instances.
pub struct ListProductInstancesRequest {
    pub owner_id: UserId,
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
