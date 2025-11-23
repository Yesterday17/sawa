use chrono::Utc;
use sawa_core::{
    errors::RepositoryError,
    models::purchase::{PurchaseOrder, PurchaseOrderItemStatus, PurchaseOrderStatus},
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
        // Load order
        let mut order = self
            .order
            .find_by_id(&req.order_id, &req.user_id)
            .await?
            .ok_or(FulfillOrderError::OrderNotFound)?;

        // Validate order is ready for fulfillment
        match order.status {
            PurchaseOrderStatus::Incomplete => {}
            PurchaseOrderStatus::Fulfilled => {
                return Ok(order);
            }
            PurchaseOrderStatus::Cancelled => {
                return Err(FulfillOrderError::OrderCancelled);
            }
        }

        // Validate all items are in Pending status
        for item in &order.items {
            if item.status != PurchaseOrderItemStatus::Pending {
                return Err(FulfillOrderError::ItemNotPending);
            }
        }

        // Create ProductInstances for all line items
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
                    Err(RepositoryError::Duplicated(e)) => {
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
                                RepositoryError::Duplicated(e),
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

        // Update order status
        order.status = PurchaseOrderStatus::Fulfilled;
        order.completed_at = Some(Utc::now());

        // Save order
        self.order.save(&order).await?;

        Ok(order)
    }

    // TODO: REVIEW IMPLEMENTATION
    async fn cancel_order(
        &self,
        req: &sawa_core::services::CancelOrderRequest,
    ) -> Result<PurchaseOrder, CancelOrderError> {
        // Load order
        let mut order = self
            .order
            .find_by_id(&req.order_id, &req.user_id)
            .await?
            .ok_or(CancelOrderError::OrderNotFound)?;

        // Only creator can cancel the order
        if order.creator_id != req.user_id {
            return Err(CancelOrderError::PermissionDenied {
                user_id: req.user_id,
            });
        }

        // Validate order can be cancelled
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

        // Update order status
        order.status = PurchaseOrderStatus::Cancelled;
        order.cancelled_at = Some(Utc::now());

        // Update all items to cancelled
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

        // Save order
        self.order.save(&order).await?;

        Ok(order)
    }
}
