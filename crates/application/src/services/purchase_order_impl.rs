use chrono::Utc;
use sawa_core::{
    models::{
        misc::{Currency, Price},
        purchase::{
            PurchaseOrder, PurchaseOrderId, PurchaseOrderItem, PurchaseOrderItemId,
            PurchaseOrderItemStatus, PurchaseOrderLineItem, PurchaseOrderStatus,
        },
    },
    repositories::*,
    services::{
        AddOrderItemError, AddOrderItemRequest, CreateOrderError, CreateOrderRequest,
        GetOrderError, GetOrderRequest, PurchaseOrderService, SubmitMysteryBoxResultsError,
        SubmitMysteryBoxResultsRequest,
    },
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> PurchaseOrderService for Service<P, PV, PI, PO, UT, U, T, M>
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
    async fn create_order(
        &self,
        req: CreateOrderRequest,
    ) -> Result<PurchaseOrder, CreateOrderError> {
        // Verify user exists
        self.user
            .find_by_id(&req.user_id)
            .await?
            .ok_or(CreateOrderError::UserNotFound {
                user_id: req.user_id,
            })?;

        // Determine receiver_id (default to creator if not specified)
        let receiver_id = req.receiver_id.unwrap_or(req.user_id);

        // If receiver_id is different from user_id, verify receiver exists
        if receiver_id != req.user_id {
            self.user
                .find_by_id(&receiver_id)
                .await?
                .ok_or(CreateOrderError::UserNotFound {
                    user_id: receiver_id,
                })?;
        }

        // Create new order
        let order = PurchaseOrder {
            id: PurchaseOrderId::new(),
            creator_id: req.user_id,
            receiver_id,
            items: vec![],
            shipping_address: req.shipping_address,
            total_price: req.total_price.unwrap_or_else(|| Price {
                currency: Currency::JPY,
                amount: 0,
            }),
            status: PurchaseOrderStatus::Incomplete,
            created_at: Utc::now(),
            completed_at: None,
            cancelled_at: None,
        };

        // Save order
        self.order.save(&order).await?;

        Ok(order)
    }

    async fn add_order_item(
        &self,
        req: AddOrderItemRequest,
    ) -> Result<PurchaseOrderItemId, AddOrderItemError> {
        // Load and verify order exists
        let mut order = self
            .order
            .find_by_id(&req.order_id, &req.user_id)
            .await?
            .ok_or(AddOrderItemError::OrderNotFound {
                order_id: req.order_id,
            })?;

        // Verify user has permission to modify this order
        if order.creator_id != req.user_id {
            return Err(AddOrderItemError::PermissionDenied {
                user_id: req.user_id,
            });
        }

        // Verify order is not already completed
        if order.status != PurchaseOrderStatus::Incomplete {
            return Err(AddOrderItemError::OrderNotEditable);
        }

        // Verify variant exists
        let variant = self
            .product_variant
            .find_by_id(&req.variant_id)
            .await?
            .ok_or(AddOrderItemError::VariantNotFound {
                variant_id: req.variant_id,
            })?;

        // Create order item
        let item_id = PurchaseOrderItemId::new();
        let is_mystery_box = variant.mystery_box.is_some();

        // Create line items based on item type
        let line_items = if is_mystery_box {
            // Mystery box: no line items yet, waiting for user input
            vec![]
        } else {
            // Regular item: create line items for each quantity
            (0..req.quantity.get())
                .map(|_| PurchaseOrderLineItem::new(req.variant_id, item_id, req.owner_id))
                .collect()
        };

        let item_status = if is_mystery_box {
            PurchaseOrderItemStatus::AwaitingInput
        } else {
            PurchaseOrderItemStatus::Pending
        };

        let item = PurchaseOrderItem {
            id: item_id,
            purchased_variant_id: req.variant_id,
            line_items,
            status: item_status,
            quantity: req.quantity,
            unit_price: req.unit_price,
        };

        // Add item to order
        order.items.push(item);

        // Update total price
        if let Some(unit_price) = req.unit_price {
            // Ensure currency matches
            if unit_price.currency != order.total_price.currency {
                return Err(AddOrderItemError::CurrencyMismatch {
                    expected: order.total_price.currency,
                    actual: unit_price.currency,
                });
            }

            // Add to total: unit_price * quantity
            let item_total = unit_price.amount as u64 * req.quantity.get() as u64;
            order.total_price.amount = order.total_price.amount.saturating_add(item_total as u32);
        }

        // Save updated order
        self.order.save(&order).await?;

        Ok(item_id)
    }

    async fn submit_mystery_box_results(
        &self,
        req: SubmitMysteryBoxResultsRequest,
    ) -> Result<(), SubmitMysteryBoxResultsError> {
        // Load and verify order exists
        let mut order = self
            .order
            .find_by_id(&req.order_id, &req.user_id)
            .await?
            .ok_or(SubmitMysteryBoxResultsError::OrderNotFound)?;

        // Verify user has permission to modify this order
        if order.creator_id != req.user_id {
            return Err(SubmitMysteryBoxResultsError::PermissionDenied {
                user_id: req.user_id,
            });
        }

        // Find the order item
        let item = order
            .items
            .iter_mut()
            .find(|item| item.id == req.order_item_id)
            .ok_or(SubmitMysteryBoxResultsError::OrderItemNotFound {
                order_item_id: req.order_item_id,
            })?;

        // Verify it's a mystery box item
        let variant = self
            .product_variant
            .find_by_id(&item.purchased_variant_id)
            .await?
            .ok_or(SubmitMysteryBoxResultsError::VariantNotFound {
                variant_id: item.purchased_variant_id,
            })?;

        let mystery_box_config = variant
            .mystery_box
            .ok_or(SubmitMysteryBoxResultsError::NotMysteryBox)?;

        // Verify the number of variants matches expected count
        let expected_count = item.quantity.get() * mystery_box_config.items_count.get();
        if req.received_variants.len() as u32 != expected_count {
            return Err(SubmitMysteryBoxResultsError::InvalidVariantCount {
                expected: expected_count,
                actual: req.received_variants.len() as u32,
            });
        }

        // Create line items for each received variant
        item.line_items = req
            .received_variants
            .into_iter()
            .map(|variant_id| PurchaseOrderLineItem::new(variant_id, item.id, req.owner_id))
            .collect();

        // Update item status to Pending
        item.status = PurchaseOrderItemStatus::Pending;

        // Save updated order
        self.order.save(&order).await?;

        Ok(())
    }

    async fn get_order(&self, req: GetOrderRequest) -> Result<PurchaseOrder, GetOrderError> {
        let order = self
            .order
            .find_by_id(&req.order_id, &req.user_id)
            .await?
            .ok_or(GetOrderError::NotFound)?;

        Ok(order)
    }
}
