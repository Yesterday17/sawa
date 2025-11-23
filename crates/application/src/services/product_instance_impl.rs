use chrono::Utc;
use sawa_core::{
    models::product::{
        ProductInstance, ProductInstanceStatus, ProductInstanceStatusHistory,
        ProductInstanceStatusHistoryId,
    },
    repositories::*,
    services::{
        ConsumeProductInstanceError, GetProductInstanceError, ListProductInstancesError,
        ListProductInstancesQueryBy, MarkProductInstanceDestroyedError,
        MarkProductInstanceLostError, ProductInstanceService,
    },
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> ProductInstanceService for Service<P, PV, PI, PO, UT, U, T, M>
where
    P: ProductRepository,
    PV: ProductVariantRepository,
    PI: ProductInstanceRepository,
    PO: PurchaseOrderRepository,
    UT: UserTransactionRepository,
    U: UserRepository,
    T: TagRepository,
    M: MediaRepository,
{
    async fn get_product_instance(
        &self,
        req: sawa_core::services::GetProductInstanceRequest,
    ) -> Result<ProductInstance, GetProductInstanceError> {
        self.product_instance
            .find_by_id(&req.id)
            .await?
            .ok_or(GetProductInstanceError::NotFound)
    }

    async fn list_product_instances(
        &self,
        req: sawa_core::services::ListProductInstancesRequest,
    ) -> Result<Vec<ProductInstance>, ListProductInstancesError> {
        let instances = match req.query_by {
            ListProductInstancesQueryBy::Owned => {
                if let Some(variant_id) = req.variant_id {
                    let mut instances = self
                        .product_instance
                        .find_by_owner_and_variant(&req.user_id, &variant_id)
                        .await?;

                    if let Some(status) = req.status {
                        instances.retain(|i| i.status == status);
                    }
                    instances
                } else if let Some(status) = req.status {
                    self.product_instance
                        .find_by_owner_and_status(&req.user_id, status)
                        .await?
                } else {
                    self.product_instance.find_by_owner(&req.user_id).await?
                }
            }
            ListProductInstancesQueryBy::Held => {
                if let Some(variant_id) = req.variant_id {
                    let mut instances = self
                        .product_instance
                        .find_by_holder_and_variant(&req.user_id, &variant_id)
                        .await?;

                    if let Some(status) = req.status {
                        instances.retain(|i| i.status == status);
                    }
                    instances
                } else if let Some(status) = req.status {
                    self.product_instance
                        .find_by_holder_and_status(&req.user_id, status)
                        .await?
                } else {
                    self.product_instance.find_by_holder(&req.user_id).await?
                }
            }
        };
        Ok(instances)
    }

    async fn consume_product_instance(
        &self,
        req: sawa_core::services::ConsumeProductInstanceRequest,
    ) -> Result<ProductInstance, ConsumeProductInstanceError> {
        let mut instance = self
            .product_instance
            .find_by_id(&req.id)
            .await?
            .ok_or(ConsumeProductInstanceError::NotFound)?;

        if instance.owner_id != req.user_id {
            return Err(ConsumeProductInstanceError::PermissionDenied);
        }

        if instance.status != ProductInstanceStatus::Active {
            return Err(ConsumeProductInstanceError::NotActive);
        }

        if instance.owner_id != instance.holder_id {
            return Err(ConsumeProductInstanceError::NotHeldByOwner);
        }

        instance.status = ProductInstanceStatus::Consumed;
        instance.status_history.push(ProductInstanceStatusHistory {
            id: ProductInstanceStatusHistoryId::new(),
            status: ProductInstanceStatus::Consumed,
            changed_at: Utc::now(),
            reason: None,
        });

        self.product_instance.save(&instance).await?;

        Ok(instance)
    }

    async fn mark_product_instance_lost(
        &self,
        req: sawa_core::services::MarkProductInstanceLostRequest,
    ) -> Result<ProductInstance, MarkProductInstanceLostError> {
        let mut instance = self
            .product_instance
            .find_by_id(&req.id)
            .await?
            .ok_or(MarkProductInstanceLostError::NotFound)?;

        if instance.owner_id != req.user_id {
            return Err(MarkProductInstanceLostError::PermissionDenied);
        }

        if instance.status != ProductInstanceStatus::Active {
            return Err(MarkProductInstanceLostError::NotActive);
        }

        if instance.owner_id != instance.holder_id {
            return Err(MarkProductInstanceLostError::NotHeldByOwner);
        }

        instance.status = ProductInstanceStatus::NotFound;
        instance.status_history.push(ProductInstanceStatusHistory {
            id: ProductInstanceStatusHistoryId::new(),
            status: ProductInstanceStatus::NotFound,
            changed_at: Utc::now(),
            reason: req.reason,
        });

        self.product_instance.save(&instance).await?;

        Ok(instance)
    }

    async fn mark_product_instance_destroyed(
        &self,
        req: sawa_core::services::MarkProductInstanceDestroyedRequest,
    ) -> Result<ProductInstance, MarkProductInstanceDestroyedError> {
        let mut instance = self
            .product_instance
            .find_by_id(&req.id)
            .await?
            .ok_or(MarkProductInstanceDestroyedError::NotFound)?;

        if instance.owner_id != req.user_id {
            return Err(MarkProductInstanceDestroyedError::PermissionDenied);
        }

        if instance.status != ProductInstanceStatus::Active {
            return Err(MarkProductInstanceDestroyedError::NotActive);
        }

        if instance.owner_id != instance.holder_id {
            return Err(MarkProductInstanceDestroyedError::NotHeldByOwner);
        }

        instance.status = ProductInstanceStatus::Destroyed;
        instance.status_history.push(ProductInstanceStatusHistory {
            id: ProductInstanceStatusHistoryId::new(),
            status: ProductInstanceStatus::Destroyed,
            changed_at: Utc::now(),
            reason: req.reason,
        });

        self.product_instance.save(&instance).await?;

        Ok(instance)
    }
}
