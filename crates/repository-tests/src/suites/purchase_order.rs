use sawa_core::{
    models::{
        misc::{Currency, Price},
        purchase::{PurchaseOrder, PurchaseOrderId, PurchaseOrderStatus},
        user::UserId,
    },
    repositories::PurchaseOrderRepository,
};

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
    let order = create_test_order(
        UserId::new(),
        UserId::new(),
        PurchaseOrderStatus::Incomplete,
    );
    let order_id = order.id;

    repo.save(&order).await.unwrap();

    let found = repo.find_by_id(&order_id, &order.creator_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, order_id);

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
    let user_orders = repo.find_by_user(&user_id, None).await.unwrap();
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
        .find_by_user(&user_id, Some(PurchaseOrderStatus::Incomplete))
        .await
        .unwrap();
    assert_eq!(incomplete_orders.len(), 1);
    assert_eq!(incomplete_orders[0].status, PurchaseOrderStatus::Incomplete);

    // Query fulfilled only
    let fulfilled_orders = repo
        .find_by_user(&user_id, Some(PurchaseOrderStatus::Fulfilled))
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
    let user_a_orders = repo.find_by_user(&user_a, None).await.unwrap();
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
    let user_b_orders = repo.find_by_user(&user_b, None).await.unwrap();
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
        .find_by_user(&user_a, Some(PurchaseOrderStatus::Incomplete))
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].creator_id, user_a);
    assert!(results.iter().all(|o| o.creator_id != user_b));

    // Clean up
    repo.delete(&order_a.id).await.unwrap();
    repo.delete(&order_b.id).await.unwrap();
}
