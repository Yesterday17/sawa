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
#[derive(Clone)]
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
        user_id: &UserId,
    ) -> Result<Option<PurchaseOrder>, RepositoryError> {
        let orders = self.orders.read().unwrap();
        let order = orders.get(id);
        if let Some(o) = order {
            if &o.creator_id == user_id || &o.receiver_id == user_id {
                return Ok(Some(o.clone()));
            }

            // Check if user owns any line item
            for item in &o.items {
                for line_item in &item.line_items {
                    if &line_item.owner_id == user_id {
                        return Ok(Some(o.clone()));
                    }
                }
            }
        }

        Ok(None)
    }

    async fn find_by_user(
        &self,
        user_id: &UserId,
        status: Option<PurchaseOrderStatus>,
    ) -> Result<Vec<PurchaseOrder>, RepositoryError> {
        let orders = self.orders.read().unwrap();
        Ok(orders
            .values()
            .filter(|o| {
                (o.creator_id == *user_id || o.receiver_id == *user_id)
                    && status.map_or(true, |s| o.status == s)
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
