use std::collections::HashMap;

use chrono::Utc;
use sawa_core::{
    errors::RepositoryError,
    models::{
        product::ProductInstanceId,
        purchase::{PurchaseOrder, PurchaseOrderItemStatus, PurchaseOrderStatus},
        transfer::{TransactionStatus, UserTransaction, UserTransactionId},
        user::UserId,
    },
    repositories::*,
    services::{CancelOrderError, FulfillOrderError, PurchaseOrderLifecycleService},
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> PurchaseOrderLifecycleService
    for Service<P, PV, PI, PO, UT, U, T, M>
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
    async fn fulfill_order(
        &self,
        req: &sawa_core::services::FulfillOrderRequest,
    ) -> Result<PurchaseOrder, FulfillOrderError> {
        // 1. Load order
        let mut order = self
            .order
            .find_by_id(&req.order_id)
            .await?
            .ok_or(FulfillOrderError::OrderNotFound)?;

        // 2. Check permission
        if order.creator_id != req.user_id && order.receiver_id != req.user_id {
            return Err(FulfillOrderError::PermissionDenied {
                user_id: req.user_id,
            });
        }

        // 3. Validate order is ready for fulfillment
        match order.status {
            PurchaseOrderStatus::Incomplete => {}
            PurchaseOrderStatus::Fulfilled => {
                return Ok(order);
            }
            PurchaseOrderStatus::Cancelled => {
                return Err(FulfillOrderError::OrderCancelled);
            }
        }

        // 4. Validate all items are in Pending status
        for item in &order.items {
            if item.status != PurchaseOrderItemStatus::Pending {
                return Err(FulfillOrderError::ItemNotPending);
            }
        }

        // 5. Create ProductInstances for all line items
        let order_owner_id = order.creator_id;
        let receiver_id = order.receiver_id;

        for item in &mut order.items {
            for line_item in &mut item.line_items {
                // Skip if already fulfilled items
                if line_item.is_fulfilled() {
                    continue;
                }

                // Create ProductInstance
                // The instance is first assigned to the receiver
                let instance = line_item.to_product_instance(receiver_id);
                match self.product_instance.save(&instance).await {
                    Ok(_) => {
                        // Update line item on success
                        line_item.fulfill(&instance);
                    }
                    Err(RepositoryError::Duplicated { field, value }) => {
                        let instance = self
                            .product_instance
                            .find_by_line_item_id(line_item.id())
                            .await?;
                        if let Some(instance) = instance {
                            // Update line item on success
                            line_item.fulfill(&instance);
                        } else {
                            // Unique constraint violated but instance not found.
                            // Populate error
                            return Err(FulfillOrderError::Repository(
                                RepositoryError::Duplicated { field, value },
                            ));
                        }
                    }
                    Err(e) => {
                        return Err(FulfillOrderError::Repository(e));
                    }
                };
            }

            // Update item status
            item.status = PurchaseOrderItemStatus::Fulfilled;
        }
        // 5.1 Save order after item updates
        self.order.save(&order).await?;

        // 6. Create UserTransactions for items designated for other users
        // 6.1 Store created transactions
        let transactions = self
            .create_transfer_transactions(order_owner_id, receiver_id, order.items.as_slice())
            .await?;

        // 6.2 Update transaction IDs in line items
        for item in &mut order.items {
            for line_item in &mut item.line_items {
                if let Some(transaction_id) = line_item.instance_id().and_then(|instance_id| {
                    transactions
                        .iter()
                        .find(|tx| tx.items.contains(&instance_id))
                        .map(|tx| tx.id)
                }) {
                    line_item.set_transfer_transaction_id(transaction_id);
                }
            }
        }

        // 7. Update order status
        order.status = PurchaseOrderStatus::Fulfilled;
        order.completed_at = Some(Utc::now());

        // 8. Save order
        self.order.save(&order).await?;

        Ok(order)
    }

    // TODO: REVIEW IMPLEMENTATION
    async fn cancel_order(
        &self,
        req: &sawa_core::services::CancelOrderRequest,
    ) -> Result<PurchaseOrder, CancelOrderError> {
        // 1. Load order
        let mut order = self
            .order
            .find_by_id(&req.order_id)
            .await?
            .ok_or(CancelOrderError::OrderNotFound)?;

        // 2. Check permission
        if order.creator_id != req.user_id && order.receiver_id != req.user_id {
            return Err(CancelOrderError::PermissionDenied {
                user_id: req.user_id,
            });
        }

        // 3. Validate order can be cancelled
        match order.status {
            PurchaseOrderStatus::Incomplete => {}
            PurchaseOrderStatus::Fulfilled => {
                return Err(CancelOrderError::OrderAlreadyCompleted);
            }
            PurchaseOrderStatus::Cancelled => {
                // Already cancelled, just return
                return Ok(order);
            }
        }

        // 4. Update order status
        order.status = PurchaseOrderStatus::Cancelled;
        order.cancelled_at = Some(Utc::now());

        // 5. Update all items to cancelled
        for item in &mut order.items {
            match item.status {
                PurchaseOrderItemStatus::Fulfilled => {
                    // Should not happen if order is Incomplete
                    // But handle defensively
                }
                _ => {
                    item.status = PurchaseOrderItemStatus::Cancelled;
                }
            }
        }

        // 5. Save order
        self.order.save(&order).await?;

        Ok(order)
    }
}

impl<P, PV, PI, PO, UT, U, T, M> Service<P, PV, PI, PO, UT, U, T, M>
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
    /// Create UserTransactions for items designated for other users.
    ///
    /// Groups instances by target owner and creates one transaction per owner.
    async fn create_transfer_transactions(
        &self,
        order_owner_id: UserId,
        receiver_id: UserId,
        items: &[sawa_core::models::purchase::PurchaseOrderItem],
    ) -> Result<Vec<UserTransaction>, FulfillOrderError> {
        let mut owner_groups: HashMap<UserId, Vec<ProductInstanceId>> = HashMap::new();

        // Group instances by target owner
        for item in items {
            for line_item in &item.line_items {
                let target_owner = line_item.target_owner_id(order_owner_id);
                if let Some(instance_id) =
                    line_item.transaction_product_instance_id(receiver_id, order_owner_id)
                {
                    owner_groups
                        .entry(target_owner)
                        .or_insert_with(Vec::new)
                        .push(instance_id);
                }
            }
        }

        let transactions = owner_groups
            .iter()
            .map(|(target_owner, instance_ids)| UserTransaction {
                id: UserTransactionId::new(),
                from_user_id: receiver_id,
                to_user_id: *target_owner,
                items: instance_ids.clone(),
                price: None,
                status: TransactionStatus::Pending,
                created_at: Utc::now(),
                completed_at: None,
                cancelled_at: None,
            })
            .collect::<Vec<_>>();
        self.transaction.save_batch(&transactions).await?;

        Ok(transactions)
    }
}
