use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::{
        product::{ProductInstance, ProductInstanceId, ProductInstanceStatus, ProductVariantId},
        purchase::PurchaseOrderLineItemId,
        user::UserId,
    },
    repositories::ProductInstanceRepository,
};

/// In-memory implementation of ProductInstanceRepository.
#[derive(Clone)]
pub struct InMemoryProductInstanceRepository {
    instances: Arc<RwLock<HashMap<ProductInstanceId, ProductInstance>>>,
}

impl InMemoryProductInstanceRepository {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryProductInstanceRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductInstanceRepository for InMemoryProductInstanceRepository {
    async fn find_by_id(
        &self,
        id: &ProductInstanceId,
    ) -> Result<Option<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances.get(id).cloned())
    }

    async fn find_by_line_item_id(
        &self,
        line_item_id: &PurchaseOrderLineItemId,
    ) -> Result<Option<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        for instance in instances.values() {
            if &instance.source_order_line_item_id == line_item_id {
                return Ok(Some(instance.clone()));
            }
        }
        Ok(None)
    }

    async fn find_by_owner(
        &self,
        owner_id: &UserId,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances
            .values()
            .filter(|i| i.owner_id == *owner_id)
            .cloned()
            .collect())
    }

    async fn find_by_owner_and_variant(
        &self,
        owner_id: &UserId,
        variant_id: &ProductVariantId,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances
            .values()
            .filter(|i| i.owner_id == *owner_id && i.variant_id == *variant_id)
            .cloned()
            .collect())
    }

    async fn find_by_owner_and_status(
        &self,
        owner_id: &UserId,
        status: ProductInstanceStatus,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances
            .values()
            .filter(|i| i.owner_id == *owner_id && i.status == status)
            .cloned()
            .collect())
    }

    async fn find_by_holder(
        &self,
        holder_id: &UserId,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances
            .values()
            .filter(|i| i.holder_id == *holder_id)
            .cloned()
            .collect())
    }

    async fn find_by_holder_and_variant(
        &self,
        holder_id: &UserId,
        variant_id: &ProductVariantId,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances
            .values()
            .filter(|i| i.holder_id == *holder_id && i.variant_id == *variant_id)
            .cloned()
            .collect())
    }

    async fn find_by_holder_and_status(
        &self,
        holder_id: &UserId,
        status: ProductInstanceStatus,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let instances = self.instances.read().unwrap();
        Ok(instances
            .values()
            .filter(|i| i.holder_id == *holder_id && i.status == status)
            .cloned()
            .collect())
    }

    async fn save(&self, instance: &ProductInstance) -> Result<(), RepositoryError> {
        let mut instances = self.instances.write().unwrap();
        instances.insert(instance.id, instance.clone());
        Ok(())
    }

    async fn save_batch(&self, instances: &[ProductInstance]) -> Result<(), RepositoryError> {
        let mut store = self.instances.write().unwrap();
        for instance in instances {
            store.insert(instance.id, instance.clone());
        }
        Ok(())
    }

    async fn delete(&self, id: &ProductInstanceId) -> Result<(), RepositoryError> {
        let mut instances = self.instances.write().unwrap();
        instances.remove(id);
        Ok(())
    }
}
