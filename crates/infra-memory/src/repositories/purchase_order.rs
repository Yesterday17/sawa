use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use sawa_core::{
    errors::RepositoryError,
    models::{
        purchase::{PurchaseOrder, PurchaseOrderId, PurchaseOrderStatus},
        user::UserId,
    },
    repositories::PurchaseOrderRepository,
};

/// In-memory implementation of PurchaseOrderRepository.
pub struct InMemoryPurchaseOrderRepository {
    orders: Arc<RwLock<HashMap<PurchaseOrderId, PurchaseOrder>>>,
}

impl InMemoryPurchaseOrderRepository {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryPurchaseOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PurchaseOrderRepository for InMemoryPurchaseOrderRepository {
    async fn find_by_id(
        &self,
        id: &PurchaseOrderId,
    ) -> Result<Option<PurchaseOrder>, RepositoryError> {
        let orders = self.orders.read().unwrap();
        Ok(orders.get(id).cloned())
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<PurchaseOrder>, RepositoryError> {
        let orders = self.orders.read().unwrap();
        Ok(orders
            .values()
            .filter(|o| o.creator_id == *user_id || o.receiver_id == *user_id)
            .cloned()
            .collect())
    }

    async fn find_by_user_and_status(
        &self,
        user_id: &UserId,
        status: PurchaseOrderStatus,
    ) -> Result<Vec<PurchaseOrder>, RepositoryError> {
        let orders = self.orders.read().unwrap();
        Ok(orders
            .values()
            .filter(|o| {
                (o.creator_id == *user_id || o.receiver_id == *user_id) && o.status == status
            })
            .cloned()
            .collect())
    }

    async fn find_incomplete_by_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<PurchaseOrder>, RepositoryError> {
        let orders = self.orders.read().unwrap();
        Ok(orders
            .values()
            .filter(|o| {
                (o.creator_id == *user_id || o.receiver_id == *user_id)
                    && matches!(o.status, PurchaseOrderStatus::Incomplete)
            })
            .cloned()
            .collect())
    }

    async fn save(&self, order: &PurchaseOrder) -> Result<(), RepositoryError> {
        let mut orders = self.orders.write().unwrap();
        orders.insert(order.id, order.clone());
        Ok(())
    }

    async fn delete(&self, id: &PurchaseOrderId) -> Result<(), RepositoryError> {
        let mut orders = self.orders.write().unwrap();
        orders.remove(id);
        Ok(())
    }
}
