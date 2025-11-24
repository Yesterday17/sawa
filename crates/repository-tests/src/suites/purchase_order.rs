use sawa_core::{
    models::{
        misc::{Currency, Price},
        product::ProductVariantId,
        purchase::{
            OrderRoleFilter, PurchaseOrder, PurchaseOrderId, PurchaseOrderItem,
            PurchaseOrderItemId, PurchaseOrderItemStatus, PurchaseOrderLineItem,
            PurchaseOrderStatus,
        },
        user::UserId,
    },
    repositories::PurchaseOrderRepository,
};
use std::num::NonZeroU32;

fn create_test_order(
    creator_id: UserId,
    receiver_id: UserId,
    status: PurchaseOrderStatus,
) -> PurchaseOrder {
    let (completed_at, cancelled_at) = match status {
        PurchaseOrderStatus::Fulfilled => (Some(chrono::Utc::now()), None),
        PurchaseOrderStatus::Cancelled => (None, Some(chrono::Utc::now())),
        _ => (None, None),
    };

    PurchaseOrder {
        id: PurchaseOrderId::new(),
        creator_id,
        receiver_id,
        items: vec![],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 1000,
        },
        status,
        created_at: chrono::Utc::now(),
        completed_at,
        cancelled_at,
    }
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: PurchaseOrderRepository>(repo: R) {
    let mut order = create_test_order(
        UserId::new(),
        UserId::new(),
        PurchaseOrderStatus::Incomplete,
    );

    // Add items and line items
    let variant_id = ProductVariantId::new();
    let item_id = PurchaseOrderItemId::new();
    let line_item_owner = UserId::new();

    let line_item = PurchaseOrderLineItem::new(variant_id, item_id, line_item_owner);

    let item = PurchaseOrderItem {
        id: item_id,
        purchased_variant_id: variant_id,
        line_items: vec![line_item],
        status: PurchaseOrderItemStatus::Pending,
        quantity: NonZeroU32::new(1).unwrap(),
        unit_price: None,
    };

    order.items.push(item);
    let order_id = order.id;

    repo.save(&order).await.unwrap();

    let found = repo.find_by_id(&order_id, &order.creator_id).await.unwrap();
    assert!(found.is_some());
    let found_order = found.unwrap();
    assert_eq!(found_order.id, order_id);

    // Verify items
    assert_eq!(found_order.items.len(), 1);
    let found_item = &found_order.items[0];
    assert_eq!(found_item.id, item_id);
    assert_eq!(found_item.purchased_variant_id, variant_id);

    // Verify line items
    assert_eq!(found_item.line_items.len(), 1);
    let found_line_item = &found_item.line_items[0];
    assert_eq!(found_line_item.variant_id, variant_id);
    assert_eq!(found_line_item.owner_id, line_item_owner);

    // Clean up
    repo.delete(&order_id).await.unwrap();
}

/// Test find_by_user without status filter returns all statuses.
pub async fn test_find_by_user_without_status_filter<R: PurchaseOrderRepository>(repo: R) {
    let user_id = UserId::new();
    let order1 = create_test_order(user_id, user_id, PurchaseOrderStatus::Incomplete);
    let order2 = create_test_order(user_id, user_id, PurchaseOrderStatus::Fulfilled);

    repo.save(&order1).await.unwrap();
    repo.save(&order2).await.unwrap();

    // Query without status filter should return both orders
    let user_orders = repo
        .find_by_user(&user_id, OrderRoleFilter::Creator, None)
        .await
        .unwrap();
    assert_eq!(user_orders.len(), 2);

    // Clean up
    repo.delete(&order1.id).await.unwrap();
    repo.delete(&order2.id).await.unwrap();
}

/// Test find_by_user with status filter.
pub async fn test_find_by_user_with_status<R: PurchaseOrderRepository>(repo: R) {
    let user_id = UserId::new();
    let incomplete = create_test_order(user_id, user_id, PurchaseOrderStatus::Incomplete);
    let fulfilled = create_test_order(user_id, user_id, PurchaseOrderStatus::Fulfilled);

    repo.save(&incomplete).await.unwrap();
    repo.save(&fulfilled).await.unwrap();

    // Query incomplete only
    let incomplete_orders = repo
        .find_by_user(
            &user_id,
            OrderRoleFilter::Creator,
            Some(PurchaseOrderStatus::Incomplete),
        )
        .await
        .unwrap();
    assert_eq!(incomplete_orders.len(), 1);
    assert_eq!(incomplete_orders[0].status, PurchaseOrderStatus::Incomplete);

    // Query fulfilled only
    let fulfilled_orders = repo
        .find_by_user(
            &user_id,
            OrderRoleFilter::Creator,
            Some(PurchaseOrderStatus::Fulfilled),
        )
        .await
        .unwrap();
    assert_eq!(fulfilled_orders.len(), 1);
    assert_eq!(fulfilled_orders[0].status, PurchaseOrderStatus::Fulfilled);

    // Clean up
    repo.delete(&incomplete.id).await.unwrap();
    repo.delete(&fulfilled.id).await.unwrap();
}

/// Test delete removes order.
pub async fn test_delete<R: PurchaseOrderRepository>(repo: R) {
    let order = create_test_order(
        UserId::new(),
        UserId::new(),
        PurchaseOrderStatus::Incomplete,
    );
    let order_id = order.id;

    repo.save(&order).await.unwrap();
    repo.delete(&order_id).await.unwrap();

    let after_delete = repo.find_by_id(&order_id, &order.creator_id).await.unwrap();
    assert!(after_delete.is_none());
}

/// Test find_by_user permission isolation (all statuses).
pub async fn test_find_by_user_permission_isolation<R: PurchaseOrderRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();

    let order_a1 = PurchaseOrder {
        id: PurchaseOrderId::new(),
        creator_id: user_a,
        receiver_id: user_a,
        items: vec![],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 1000,
        },
        status: PurchaseOrderStatus::Incomplete,
        created_at: chrono::Utc::now(),
        completed_at: None,
        cancelled_at: None,
    };

    let order_a2 = PurchaseOrder {
        id: PurchaseOrderId::new(),
        creator_id: user_a,
        receiver_id: user_a,
        items: vec![],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 2000,
        },
        status: PurchaseOrderStatus::Fulfilled,
        created_at: chrono::Utc::now(),
        completed_at: Some(chrono::Utc::now()),
        cancelled_at: None,
    };

    let order_b = PurchaseOrder {
        id: PurchaseOrderId::new(),
        creator_id: user_b,
        receiver_id: user_b,
        items: vec![],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 500,
        },
        status: PurchaseOrderStatus::Incomplete,
        created_at: chrono::Utc::now(),
        completed_at: None,
        cancelled_at: None,
    };

    repo.save(&order_a1).await.unwrap();
    repo.save(&order_a2).await.unwrap();
    repo.save(&order_b).await.unwrap();

    // User A should only see their own orders (query all statuses)
    let user_a_orders = repo
        .find_by_user(&user_a, OrderRoleFilter::Participant, None)
        .await
        .unwrap();
    assert_eq!(user_a_orders.len(), 2);
    assert!(
        user_a_orders
            .iter()
            .all(|o| o.creator_id == user_a || o.receiver_id == user_a)
    );
    assert!(
        user_a_orders
            .iter()
            .all(|o| o.creator_id != user_b && o.receiver_id != user_b)
    );

    // User B should only see their own order
    let user_b_orders = repo
        .find_by_user(&user_b, OrderRoleFilter::Participant, None)
        .await
        .unwrap();
    assert_eq!(user_b_orders.len(), 1);
    assert_eq!(user_b_orders[0].creator_id, user_b);

    // Clean up
    repo.delete(&order_a1.id).await.unwrap();
    repo.delete(&order_a2.id).await.unwrap();
    repo.delete(&order_b.id).await.unwrap();
}

/// Test find_by_user with status filter respects user permission.
pub async fn test_find_by_user_and_status_permission<R: PurchaseOrderRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();

    let order_a = PurchaseOrder {
        id: PurchaseOrderId::new(),
        creator_id: user_a,
        receiver_id: user_a,
        items: vec![],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 1000,
        },
        status: PurchaseOrderStatus::Incomplete,
        created_at: chrono::Utc::now(),
        completed_at: None,
        cancelled_at: None,
    };

    let order_b = PurchaseOrder {
        id: PurchaseOrderId::new(),
        creator_id: user_b,
        receiver_id: user_b,
        items: vec![],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 1000,
        },
        status: PurchaseOrderStatus::Incomplete,
        created_at: chrono::Utc::now(),
        completed_at: None,
        cancelled_at: None,
    };

    repo.save(&order_a).await.unwrap();
    repo.save(&order_b).await.unwrap();

    // User A queries for Incomplete status - should only see their own
    let results = repo
        .find_by_user(
            &user_a,
            OrderRoleFilter::Participant,
            Some(PurchaseOrderStatus::Incomplete),
        )
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].creator_id, user_a);
    assert!(results.iter().all(|o| o.creator_id != user_b));

    // Clean up
    repo.delete(&order_a.id).await.unwrap();
    repo.delete(&order_b.id).await.unwrap();
}

/// Test find_by_id access control.
///
/// Ensures that:
/// - Creator can access
/// - Receiver can access
/// - Owner of any line item can access
/// - Random user cannot access
pub async fn test_find_by_id_access_control<R: PurchaseOrderRepository>(repo: R) {
    let creator = UserId::new();
    let receiver = UserId::new();
    let item_owner = UserId::new();
    let random_user = UserId::new();

    let order_id = PurchaseOrderId::new();
    let item_id = PurchaseOrderItemId::new();
    let variant_id = ProductVariantId::new();

    let line_item = PurchaseOrderLineItem::new(variant_id, item_id, item_owner);

    let item = PurchaseOrderItem {
        id: item_id,
        purchased_variant_id: variant_id,
        line_items: vec![line_item],
        status: PurchaseOrderItemStatus::Pending,
        quantity: NonZeroU32::new(1).unwrap(),
        unit_price: None,
    };

    let order = PurchaseOrder {
        id: order_id,
        creator_id: creator,
        receiver_id: receiver,
        items: vec![item],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 1000,
        },
        status: PurchaseOrderStatus::Incomplete,
        created_at: chrono::Utc::now(),
        completed_at: None,
        cancelled_at: None,
    };

    repo.save(&order).await.unwrap();

    // Creator should find it
    assert!(
        repo.find_by_id(&order_id, &creator)
            .await
            .unwrap()
            .is_some()
    );

    // Receiver should find it
    assert!(
        repo.find_by_id(&order_id, &receiver)
            .await
            .unwrap()
            .is_some()
    );

    // Item owner should find it
    assert!(
        repo.find_by_id(&order_id, &item_owner)
            .await
            .unwrap()
            .is_some()
    );

    // Random user should NOT find it
    assert!(
        repo.find_by_id(&order_id, &random_user)
            .await
            .unwrap()
            .is_none()
    );

    // Clean up
    repo.delete(&order_id).await.unwrap();
}

/// Test find_by_user with role filter.
pub async fn test_find_by_user_role_filter<R: PurchaseOrderRepository>(repo: R) {
    let creator = UserId::new();
    let receiver = UserId::new();
    let participant = UserId::new();
    let other = UserId::new();

    let order_id = PurchaseOrderId::new();
    let item_id = PurchaseOrderItemId::new();
    let variant_id = ProductVariantId::new();

    // Participant owns a line item
    let line_item = PurchaseOrderLineItem::new(variant_id, item_id, participant);

    let item = PurchaseOrderItem {
        id: item_id,
        purchased_variant_id: variant_id,
        line_items: vec![line_item],
        status: PurchaseOrderItemStatus::Pending,
        quantity: NonZeroU32::new(1).unwrap(),
        unit_price: None,
    };

    let order = PurchaseOrder {
        id: order_id,
        creator_id: creator,
        receiver_id: receiver,
        items: vec![item],
        shipping_address: None,
        total_price: Price {
            currency: Currency::USD,
            amount: 1000,
        },
        status: PurchaseOrderStatus::Incomplete,
        created_at: chrono::Utc::now(),
        completed_at: None,
        cancelled_at: None,
    };

    repo.save(&order).await.unwrap();

    // 1. Creator role
    // Creator should find it
    let res = repo
        .find_by_user(&creator, OrderRoleFilter::Creator, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, order_id);

    // Receiver should NOT find it as Creator
    let res = repo
        .find_by_user(&receiver, OrderRoleFilter::Creator, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 0);

    // Participant should NOT find it as Creator
    let res = repo
        .find_by_user(&participant, OrderRoleFilter::Creator, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 0);

    // 2. Receiver role
    // Receiver should find it
    let res = repo
        .find_by_user(&receiver, OrderRoleFilter::Receiver, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, order_id);

    // Creator should NOT find it as Receiver
    let res = repo
        .find_by_user(&creator, OrderRoleFilter::Receiver, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 0);

    // Participant should NOT find it as Receiver
    let res = repo
        .find_by_user(&participant, OrderRoleFilter::Receiver, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 0);

    // 3. Participant role (includes Creator, Receiver, and Item Owners)
    // Creator is a participant
    let res = repo
        .find_by_user(&creator, OrderRoleFilter::Participant, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 1);

    // Receiver is a participant
    let res = repo
        .find_by_user(&receiver, OrderRoleFilter::Participant, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 1);

    // Item Owner is a participant
    let res = repo
        .find_by_user(&participant, OrderRoleFilter::Participant, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 1);

    // Random user is NOT a participant
    let res = repo
        .find_by_user(&other, OrderRoleFilter::Participant, None)
        .await
        .unwrap();
    assert_eq!(res.len(), 0);

    // Clean up
    repo.delete(&order_id).await.unwrap();
}

/// Test load_by_ids.
pub async fn test_load_by_ids<R: PurchaseOrderRepository>(repo: R) {
    let user_id = UserId::new();
    let order1 = create_test_order(user_id, user_id, PurchaseOrderStatus::Incomplete);
    let order2 = create_test_order(user_id, user_id, PurchaseOrderStatus::Fulfilled);
    let order3 = create_test_order(user_id, user_id, PurchaseOrderStatus::Cancelled);

    repo.save(&order1).await.unwrap();
    repo.save(&order2).await.unwrap();
    repo.save(&order3).await.unwrap();

    let non_existent_id = PurchaseOrderId::new();

    let ids = vec![order1.id, non_existent_id, order3.id, order2.id];

    let results = repo.load_by_ids(&ids).await.unwrap();

    assert_eq!(results.len(), 4);
    assert_eq!(results[0].as_ref().unwrap().id, order1.id);
    assert!(results[1].is_none());
    assert_eq!(results[2].as_ref().unwrap().id, order3.id);
    assert_eq!(results[3].as_ref().unwrap().id, order2.id);

    // Clean up
    repo.delete(&order1.id).await.unwrap();
    repo.delete(&order2.id).await.unwrap();
    repo.delete(&order3.id).await.unwrap();
}
