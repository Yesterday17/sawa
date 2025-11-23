mod common;

use common::{create_service, create_user};
use sawa_core::models::misc::{Address, Currency, NonEmptyString, Price};
use sawa_core::models::product::ProductInstanceStatus;
use sawa_core::repositories::*;
use sawa_core::services::*;
use std::num::NonZeroU32;

#[tokio::test]
async fn test_purchase_flow() {
    let service = create_service();

    // Setup: User, Product, Variant
    let user = create_user("test_user");
    let user = service.user.create(user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // 1. Create Order
    let order = service
        .create_order(CreateOrderRequest {
            user_id: user.id,
            receiver_id: None,
            shipping_address: Some(Address {
                line1: "123 Main St".to_string(),
                line2: None,
                city: "Tokyo".to_string(),
                state_or_province: "Tokyo".to_string(),
                postal_code: "100-0001".to_string(),
                country: "JP".to_string(),
            }),
            items: vec![],
            total_price: None,
        })
        .await
        .expect("Failed to create order");

    // 2. Add Item
    service
        .add_order_item(AddOrderItemRequest {
            user_id: user.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: user.id,
            quantity: NonZeroU32::new(2).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .expect("Failed to add item");

    // 3. Fulfill Order
    let fulfilled_order = service
        .fulfill_order(&FulfillOrderRequest {
            user_id: user.id,
            order_id: order.id,
        })
        .await
        .expect("Failed to fulfill order");

    assert_eq!(
        fulfilled_order.status,
        sawa_core::models::purchase::PurchaseOrderStatus::Fulfilled
    );

    // 4. Check Product Instances
    let instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: user.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list instances");

    assert_eq!(instances.len(), 2);
    assert!(
        instances
            .iter()
            .all(|i| i.status == ProductInstanceStatus::Active)
    );
}

#[tokio::test]
async fn test_order_fulfillment_flow() {
    let service = create_service();

    // Setup: User, Product, Variant
    let user = create_user("test_user");
    let user = service.user.create(user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // 1. Create Order (status: Incomplete)
    let order = service
        .create_order(CreateOrderRequest {
            user_id: user.id,
            receiver_id: None,
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .expect("Failed to create order");

    assert_eq!(
        order.status,
        sawa_core::models::purchase::PurchaseOrderStatus::Incomplete
    );

    // 2. Add Item (status: Pending for regular items)
    service
        .add_order_item(AddOrderItemRequest {
            user_id: user.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: user.id,
            quantity: NonZeroU32::new(2).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .expect("Failed to add item");

    // 3. Fulfill Order (status: Incomplete -> Fulfilled)
    let fulfilled_order = service
        .fulfill_order(&FulfillOrderRequest {
            user_id: user.id,
            order_id: order.id,
        })
        .await
        .expect("Failed to fulfill order");

    // Verify order status
    assert_eq!(
        fulfilled_order.status,
        sawa_core::models::purchase::PurchaseOrderStatus::Fulfilled
    );
    assert!(fulfilled_order.completed_at.is_some());

    // Verify all items are Fulfilled
    assert!(
        fulfilled_order
            .items
            .iter()
            .all(|item| item.status
                == sawa_core::models::purchase::PurchaseOrderItemStatus::Fulfilled)
    );

    // Verify ProductInstances were created
    let instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: user.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list instances");

    assert_eq!(instances.len(), 2);
    assert!(
        instances
            .iter()
            .all(|i| i.status == ProductInstanceStatus::Active)
    );

    // Verify instances are owned by the user
    assert!(instances.iter().all(|i| i.owner_id == user.id));
    assert!(instances.iter().all(|i| i.holder_id == user.id));
}

/// When a order is created with a different creator and receiver, the instances should be owned by the owner and held by the receiver.
/// Then the receiver can transfer the instances to the owner for delivery.
#[tokio::test]
async fn test_order_with_different_creator_and_receiver() {
    let service = create_service();

    // Setup: Two users
    let creator = create_user("creator");
    let receiver = create_user("receiver");
    let creator = service.user.create(creator).await.unwrap();
    let receiver = service.user.create(receiver).await.unwrap();

    // Setup: Product and Variant
    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // 1. Creator creates order with receiver specified
    let order = service
        .create_order(CreateOrderRequest {
            user_id: creator.id,
            receiver_id: Some(receiver.id),
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .expect("Failed to create order");

    // Verify order has correct creator and receiver
    assert_eq!(order.creator_id, creator.id);
    assert_eq!(order.receiver_id, receiver.id);

    // 2. Creator adds item to order
    service
        .add_order_item(AddOrderItemRequest {
            user_id: creator.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: creator.id,
            quantity: NonZeroU32::new(2).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .expect("Failed to add item");

    // 3. Either creator or receiver can fulfill the order
    // Let's test with receiver fulfilling
    let fulfilled_order = service
        .fulfill_order(&FulfillOrderRequest {
            user_id: receiver.id,
            order_id: order.id,
        })
        .await
        .expect("Failed to fulfill order");

    assert_eq!(
        fulfilled_order.status,
        sawa_core::models::purchase::PurchaseOrderStatus::Fulfilled
    );

    // 4. Verify ProductInstances are owned by creator (owner) and held by receiver
    let creator_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: creator.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list creator instances");
    assert_eq!(creator_instances.len(), 2);
    assert!(creator_instances.iter().all(|i| i.owner_id == creator.id));
    assert!(creator_instances.iter().all(|i| i.holder_id == receiver.id));
    assert!(
        creator_instances
            .iter()
            .all(|i| i.status == ProductInstanceStatus::Active)
    );

    // 5. Verify receiver has no instances (by owner)
    let receiver_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: receiver.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list receiver instances");
    assert_eq!(receiver_instances.len(), 0);

    // 6. Verify receiver holds all instances (by holder)
    let receiver_holds = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: receiver.id,
            query_by: ListProductInstancesQueryBy::Holder,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list receiver holds");
    assert_eq!(receiver_holds.len(), 2);
    assert!(receiver_holds.iter().all(|i| i.holder_id == receiver.id));
    assert!(receiver_holds.iter().all(|i| i.owner_id == creator.id));

    // 7. Verify creator does not hold any instances (by holder)
    let creator_holds = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: creator.id,
            query_by: ListProductInstancesQueryBy::Holder,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list creator holds");
    assert_eq!(creator_holds.len(), 0); // Creator doesn't hold instances, receiver does
}

#[tokio::test]
async fn test_order_cancellation_flow() {
    let service = create_service();

    // Setup: User, Product, Variant
    let user = create_user("test_user");
    let user = service.user.create(user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // 1. Create Order (status: Incomplete)
    let order = service
        .create_order(CreateOrderRequest {
            user_id: user.id,
            receiver_id: None,
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .expect("Failed to create order");

    // 2. Add Item
    service
        .add_order_item(AddOrderItemRequest {
            user_id: user.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: user.id,
            quantity: NonZeroU32::new(1).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .expect("Failed to add item");

    // 3. Cancel Order (status: Incomplete -> Cancelled)
    let cancelled_order = service
        .cancel_order(&CancelOrderRequest {
            user_id: user.id,
            order_id: order.id,
            reason: None,
        })
        .await
        .expect("Failed to cancel order");

    // Verify order status
    assert_eq!(
        cancelled_order.status,
        sawa_core::models::purchase::PurchaseOrderStatus::Cancelled
    );
    assert!(cancelled_order.cancelled_at.is_some());

    // Verify all items are Cancelled
    assert!(
        cancelled_order
            .items
            .iter()
            .all(|item| item.status
                == sawa_core::models::purchase::PurchaseOrderItemStatus::Cancelled)
    );

    // Verify no ProductInstances were created
    let instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: user.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list instances");

    assert_eq!(instances.len(), 0);
}

#[tokio::test]
async fn test_fulfill_cancelled_order_fails() {
    let service = create_service();

    // Setup
    let user = create_user("test_user");
    let user = service.user.create(user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Create and cancel order
    let order = service
        .create_order(CreateOrderRequest {
            user_id: user.id,
            receiver_id: None,
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .unwrap();

    service
        .add_order_item(AddOrderItemRequest {
            user_id: user.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: user.id,
            quantity: NonZeroU32::new(1).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .unwrap();

    service
        .cancel_order(&CancelOrderRequest {
            user_id: user.id,
            order_id: order.id,
            reason: None,
        })
        .await
        .unwrap();

    // Try to fulfill cancelled order - should fail
    let result = service
        .fulfill_order(&FulfillOrderRequest {
            user_id: user.id,
            order_id: order.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        FulfillOrderError::OrderCancelled => {}
        _ => panic!("Expected OrderCancelled error"),
    }
}

#[tokio::test]
async fn test_cancel_fulfilled_order_fails() {
    let service = create_service();

    // Setup
    let user = create_user("test_user");
    let user = service.user.create(user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Create and fulfill order
    let order = service
        .create_order(CreateOrderRequest {
            user_id: user.id,
            receiver_id: None,
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .unwrap();

    service
        .add_order_item(AddOrderItemRequest {
            user_id: user.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: user.id,
            quantity: NonZeroU32::new(1).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .unwrap();

    service
        .fulfill_order(&FulfillOrderRequest {
            user_id: user.id,
            order_id: order.id,
        })
        .await
        .unwrap();

    // Try to cancel fulfilled order - should fail
    let result = service
        .cancel_order(&CancelOrderRequest {
            user_id: user.id,
            order_id: order.id,
            reason: None,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CancelOrderError::OrderAlreadyCompleted => {}
        _ => panic!("Expected OrderAlreadyCompleted error"),
    }
}

#[tokio::test]
async fn test_fulfill_order_without_pending_items_fails() {
    let service = create_service();

    // Setup
    let user = create_user("test_user");
    let user = service.user.create(user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    // Create mystery box variant (items start as AwaitingInput)
    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("Mystery Box".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: Some(sawa_core::models::product::MysteryBoxConfig {
                items_count: NonZeroU32::new(1).unwrap(),
                possible_variants: vec![],
            }),
        })
        .await
        .unwrap();

    // Create order and add mystery box item
    let order = service
        .create_order(CreateOrderRequest {
            user_id: user.id,
            receiver_id: None,
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .unwrap();

    service
        .add_order_item(AddOrderItemRequest {
            user_id: user.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: user.id,
            quantity: NonZeroU32::new(1).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .unwrap();

    // Try to fulfill order with AwaitingInput items - should fail
    let result = service
        .fulfill_order(&FulfillOrderRequest {
            user_id: user.id,
            order_id: order.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        FulfillOrderError::ItemNotPending => {}
        _ => panic!("Expected ItemNotPending error"),
    }
}

#[tokio::test]
async fn test_fulfill_order_permission_denied() {
    let service = create_service();

    // Setup: Two users
    let creator = create_user("creator");
    let other_user = create_user("other");
    let creator = service.user.create(creator).await.unwrap();
    let other_user = service.user.create(other_user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Creator creates order
    let order = service
        .create_order(CreateOrderRequest {
            user_id: creator.id,
            receiver_id: None,
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .unwrap();

    service
        .add_order_item(AddOrderItemRequest {
            user_id: creator.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: creator.id,
            quantity: NonZeroU32::new(1).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .unwrap();

    // Other user tries to fulfill - should fail
    let result = service
        .fulfill_order(&FulfillOrderRequest {
            user_id: other_user.id,
            order_id: order.id,
        })
        .await;

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        FulfillOrderError::OrderNotFound
    ));
}

#[tokio::test]
async fn test_cancel_order_permission_denied() {
    let service = create_service();

    // Setup: Two users
    let creator = create_user("creator");
    let receiver_user = create_user("receiver");
    let creator = service.user.create(creator).await.unwrap();
    let receiver_user = service.user.create(receiver_user).await.unwrap();

    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("P1".to_string()).unwrap(),
            description: "".to_string(),
            medias: vec![],
        })
        .await
        .unwrap();

    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("V1".to_string()).unwrap(),
            description: "".to_string(),
            price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Creator creates order
    let order = service
        .create_order(CreateOrderRequest {
            user_id: creator.id,
            receiver_id: Some(receiver_user.id),
            shipping_address: None,
            total_price: None,
            items: vec![],
        })
        .await
        .unwrap();

    service
        .add_order_item(AddOrderItemRequest {
            user_id: creator.id,
            order_id: order.id,
            variant_id: variant.id,
            owner_id: creator.id,
            quantity: NonZeroU32::new(1).unwrap(),
            unit_price: Some(Price {
                currency: Currency::JPY,
                amount: 1000,
            }),
        })
        .await
        .unwrap();

    // Other user tries to cancel - should fail
    let result = service
        .cancel_order(&CancelOrderRequest {
            user_id: receiver_user.id,
            order_id: order.id,
            reason: None,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CancelOrderError::PermissionDenied { user_id } => {
            assert_eq!(user_id, receiver_user.id);
        }
        _ => panic!("Expected PermissionDenied error"),
    }
}
