use chrono::Utc;
use sawa_application::Service;
use sawa_core::models::misc::{Address, Currency, NonEmptyString, Price};
use sawa_core::models::product::{ProductInstance, ProductInstanceId, ProductInstanceStatus};
use sawa_core::models::purchase::PurchaseOrderLineItemId;
use sawa_core::models::user::{Email, User, UserId, Username};
use sawa_core::repositories::*;
use sawa_core::services::*;
use sawa_infra_memory::*;
use std::num::NonZeroU32;

type TestService = Service<
    InMemoryProductRepository,
    InMemoryProductVariantRepository,
    InMemoryProductInstanceRepository,
    InMemoryPurchaseOrderRepository,
    InMemoryUserTransactionRepository,
    InMemoryUserRepository,
    InMemoryTagRepository,
    InMemoryMediaRepository,
>;

fn create_service() -> TestService {
    Service {
        product: InMemoryProductRepository::new(),
        product_variant: InMemoryProductVariantRepository::new(),
        product_instance: InMemoryProductInstanceRepository::new(),
        order: InMemoryPurchaseOrderRepository::new(),
        transaction: InMemoryUserTransactionRepository::new(),
        user: InMemoryUserRepository::new(),
        tag: InMemoryTagRepository::new(),
        media: InMemoryMediaRepository::new(),
    }
}

fn create_user(username: &str) -> User {
    User {
        id: UserId::new(),
        username: Username(username.to_string()),
        email: Email(format!("{}@example.com", username)),
        password_hash: NonEmptyString::new("hash".to_string()).unwrap(),
        avatar: None,
        created_at: Utc::now(),
    }
}

#[tokio::test]
async fn test_product_lifecycle() {
    let service = create_service();

    // 1. Create Product
    let product = service
        .create_product(CreateProductRequest {
            name: NonEmptyString::new("Test Product".to_string()).unwrap(),
            description: "Description".to_string(),
            medias: vec![],
        })
        .await
        .expect("Failed to create product");

    assert_eq!(product.name.as_str(), "Test Product");

    // 2. Create Variant
    let variant = service
        .create_product_variant(CreateProductVariantRequest {
            product_id: product.id,
            name: NonEmptyString::new("Variant A".to_string()).unwrap(),
            description: "Desc".to_string(),
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
        .expect("Failed to create variant");

    assert_eq!(variant.name.as_str(), "Variant A");
    assert_eq!(variant.product_id, product.id);
}

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
async fn test_transaction_lifecycle() {
    let service = create_service();

    // Setup: Users
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

    // Setup: Alice owns an item
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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Manually create an instance for Alice (simulating fulfillment)
    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // 1. Create Transaction (Alice -> Bob)
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .expect("Failed to create transaction");

    // Verify item is locked
    let locked_instance = service
        .get_product_instance(GetProductInstanceRequest { id: instance.id })
        .await
        .unwrap();
    assert_eq!(locked_instance.status, ProductInstanceStatus::Locked);

    // 2. Complete Transaction (by Bob)
    service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await
        .expect("Failed to complete transaction");

    // Verify ownership transfer
    let transferred_instance = service
        .get_product_instance(GetProductInstanceRequest { id: instance.id })
        .await
        .unwrap();
    assert_eq!(transferred_instance.owner_id, bob.id);
    assert_eq!(transferred_instance.status, ProductInstanceStatus::Active);
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
    match result.unwrap_err() {
        FulfillOrderError::PermissionDenied { user_id } => {
            assert_eq!(user_id, other_user.id);
        }
        _ => panic!("Expected PermissionDenied error"),
    }
}

#[tokio::test]
async fn test_cancel_order_permission_denied() {
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
            user_id: other_user.id,
            order_id: order.id,
            reason: None,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CancelOrderError::PermissionDenied { user_id } => {
            assert_eq!(user_id, other_user.id);
        }
        _ => panic!("Expected PermissionDenied error"),
    }
}

// ========== Product Item Transfer Tests ==========

#[tokio::test]
async fn test_transaction_complete_flow() {
    let service = create_service();

    // Setup: Users
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

    // Setup: Alice owns an item
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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Manually create an instance for Alice (simulating fulfillment)
    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // 1. Create Transaction (Alice -> Bob)
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .expect("Failed to create transaction");

    // Verify transaction status
    assert_eq!(
        transaction.status,
        sawa_core::models::transfer::UserTransactionStatus::Pending
    );

    // Verify item is locked
    let locked_instance = service
        .get_product_instance(GetProductInstanceRequest { id: instance.id })
        .await
        .unwrap();
    assert_eq!(locked_instance.status, ProductInstanceStatus::Locked);
    assert_eq!(locked_instance.owner_id, alice.id); // Still owned by Alice

    // 2. Complete Transaction (by Bob - receiver)
    let completed_transaction = service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await
        .expect("Failed to complete transaction");

    // Verify transaction status
    assert_eq!(
        completed_transaction.status,
        sawa_core::models::transfer::UserTransactionStatus::Completed
    );
    assert!(completed_transaction.completed_at.is_some());

    // Verify ownership transfer
    let transferred_instance = service
        .get_product_instance(GetProductInstanceRequest { id: instance.id })
        .await
        .unwrap();
    assert_eq!(transferred_instance.owner_id, bob.id);
    assert_eq!(transferred_instance.holder_id, bob.id);
    assert_eq!(transferred_instance.status, ProductInstanceStatus::Active);
}

#[tokio::test]
async fn test_transaction_cancel_flow() {
    let service = create_service();

    // Setup: Users
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

    // Setup: Alice owns an item
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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Manually create an instance for Alice
    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // 1. Create Transaction (Alice -> Bob)
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .expect("Failed to create transaction");

    // Verify item is locked
    let locked_instance = service
        .get_product_instance(GetProductInstanceRequest { id: instance.id })
        .await
        .unwrap();
    assert_eq!(locked_instance.status, ProductInstanceStatus::Locked);

    // 2. Cancel Transaction (by Alice - sender)
    let cancelled_transaction = service
        .cancel_transaction(CancelTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await
        .expect("Failed to cancel transaction");

    // Verify transaction status
    assert_eq!(
        cancelled_transaction.status,
        sawa_core::models::transfer::UserTransactionStatus::Cancelled
    );
    assert!(cancelled_transaction.cancelled_at.is_some());

    // Verify item is unlocked and still owned by Alice
    let unlocked_instance = service
        .get_product_instance(GetProductInstanceRequest { id: instance.id })
        .await
        .unwrap();
    assert_eq!(unlocked_instance.status, ProductInstanceStatus::Active);
    assert_eq!(unlocked_instance.owner_id, alice.id);
    assert_eq!(unlocked_instance.holder_id, alice.id);
}

#[tokio::test]
async fn test_transaction_multiple_items() {
    let service = create_service();

    // Setup: Users
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

    // Setup: Product and variant
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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Create multiple instances for Alice
    let instance1 = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    let instance2 = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance1).await.unwrap();
    service.product_instance.save(&instance2).await.unwrap();

    // Create transaction with multiple items
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance1.id, instance2.id],
        })
        .await
        .expect("Failed to create transaction");

    // Verify all items are locked
    let locked1 = service
        .get_product_instance(GetProductInstanceRequest { id: instance1.id })
        .await
        .unwrap();
    let locked2 = service
        .get_product_instance(GetProductInstanceRequest { id: instance2.id })
        .await
        .unwrap();
    assert_eq!(locked1.status, ProductInstanceStatus::Locked);
    assert_eq!(locked2.status, ProductInstanceStatus::Locked);

    // Complete transaction
    service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await
        .expect("Failed to complete transaction");

    // Verify all items ownership transferred
    let transferred1 = service
        .get_product_instance(GetProductInstanceRequest { id: instance1.id })
        .await
        .unwrap();
    let transferred2 = service
        .get_product_instance(GetProductInstanceRequest { id: instance2.id })
        .await
        .unwrap();
    assert_eq!(transferred1.owner_id, bob.id);
    assert_eq!(transferred2.owner_id, bob.id);
    assert_eq!(transferred1.status, ProductInstanceStatus::Active);
    assert_eq!(transferred2.status, ProductInstanceStatus::Active);
}

#[tokio::test]
async fn test_complete_already_completed_transaction_fails() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Create and complete transaction
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .unwrap();

    service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await
        .unwrap();

    // Try to complete again - should fail
    let result = service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CompleteTransactionError::AlreadyCompleted => {}
        _ => panic!("Expected AlreadyCompleted error"),
    }
}

#[tokio::test]
async fn test_complete_cancelled_transaction_fails() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Create and cancel transaction
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .unwrap();

    service
        .cancel_transaction(CancelTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await
        .unwrap();

    // Try to complete cancelled transaction - should fail
    let result = service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CompleteTransactionError::Cancelled => {}
        _ => panic!("Expected Cancelled error"),
    }
}

#[tokio::test]
async fn test_cancel_completed_transaction_fails() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Create and complete transaction
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .unwrap();

    service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await
        .unwrap();

    // Try to cancel completed transaction - should fail
    let result = service
        .cancel_transaction(CancelTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CancelTransactionError::AlreadyCompleted => {}
        _ => panic!("Expected AlreadyCompleted error"),
    }
}

#[tokio::test]
async fn test_cancel_already_cancelled_transaction() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Create and cancel transaction
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .unwrap();

    service
        .cancel_transaction(CancelTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await
        .unwrap();

    // Try to cancel again - should fail
    let result = service
        .cancel_transaction(CancelTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CancelTransactionError::AlreadyCancelled => {}
        _ => panic!("Expected AlreadyCancelled error"),
    }
}

#[tokio::test]
async fn test_complete_transaction_permission_denied() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Create transaction (Alice -> Bob)
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .unwrap();

    // Alice (sender) tries to complete - should fail (only receiver can complete)
    let result = service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CompleteTransactionError::PermissionDenied => {}
        _ => panic!("Expected PermissionDenied error"),
    }
}

#[tokio::test]
async fn test_cancel_transaction_permission() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Create transaction (Alice -> Bob)
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .unwrap();

    // Test: Alice (sender) can cancel
    let cancelled_by_sender = service
        .cancel_transaction(CancelTransactionRequest {
            transaction_id: transaction.id,
            user_id: alice.id,
        })
        .await;

    // If first cancel succeeded, create another transaction to test receiver can also cancel
    if cancelled_by_sender.is_ok() {
        let instance2 = ProductInstance {
            id: ProductInstanceId::new(),
            variant_id: variant.id,
            owner_id: alice.id,
            holder_id: alice.id,
            status: ProductInstanceStatus::Active,
            source_order_line_item_id: PurchaseOrderLineItemId::new(),
            created_at: Utc::now(),
            transfer_history: vec![],
            status_history: vec![],
        };
        service.product_instance.save(&instance2).await.unwrap();

        let transaction2 = service
            .create_transaction(CreateTransactionRequest {
                from_user_id: alice.id,
                to_user_id: bob.id,
                items: vec![instance2.id],
            })
            .await
            .unwrap();

        // Test: Bob (receiver) can also cancel
        let cancelled_by_receiver = service
            .cancel_transaction(CancelTransactionRequest {
                transaction_id: transaction2.id,
                user_id: bob.id,
            })
            .await;

        assert!(cancelled_by_receiver.is_ok());
    } else {
        panic!("Sender should be able to cancel transaction");
    }
}

#[tokio::test]
async fn test_create_transaction_with_inactive_item_fails() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Create instance with Consumed status (not Active)
    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Consumed,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Try to create transaction with inactive item - should fail
    let result = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CreateTransactionError::ItemNotActive => {}
        _ => panic!("Expected ItemNotActive error"),
    }
}

#[tokio::test]
async fn test_create_transaction_with_non_owned_item_fails() {
    let service = create_service();

    // Setup
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Create instance owned by Bob
    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: bob.id,
        holder_id: bob.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Alice tries to create transaction with Bob's item - should fail
    let result = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        CreateTransactionError::ItemNotOwned => {}
        _ => panic!("Expected ItemNotOwned error"),
    }
}

#[tokio::test]
async fn test_list_product_instances_by_holder() {
    let service = create_service();

    // Setup: Users
    let alice = create_user("alice");
    let bob = create_user("bob");
    let alice = service.user.create(alice).await.unwrap();
    let bob = service.user.create(bob).await.unwrap();

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
            price: None,
            sort_order: 0,
            medias: vec![],
            tags: vec![],
            mystery_box: None,
        })
        .await
        .unwrap();

    // Create instance owned and held by Alice
    let instance = ProductInstance {
        id: ProductInstanceId::new(),
        variant_id: variant.id,
        owner_id: alice.id,
        holder_id: alice.id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    };
    service.product_instance.save(&instance).await.unwrap();

    // Verify: Query by owner (Alice) - should find the instance
    let owner_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: alice.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list owner instances");

    assert_eq!(owner_instances.len(), 1);
    assert_eq!(owner_instances[0].owner_id, alice.id);
    assert_eq!(owner_instances[0].holder_id, alice.id);

    // Verify: Query by holder (Alice) - should find the instance
    let holder_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: alice.id,
            query_by: ListProductInstancesQueryBy::Holder,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list holder instances");

    assert_eq!(holder_instances.len(), 1);
    assert_eq!(holder_instances[0].holder_id, alice.id);
    assert_eq!(holder_instances[0].owner_id, alice.id);

    // Create transaction (Alice -> Bob) but don't complete it yet
    // This will lock the item, holder_id remains Alice
    let transaction = service
        .create_transaction(CreateTransactionRequest {
            from_user_id: alice.id,
            to_user_id: bob.id,
            items: vec![instance.id],
        })
        .await
        .expect("Failed to create transaction");

    // Verify: Query by holder (Alice) - should still find the locked instance
    let locked_holder_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: alice.id,
            query_by: ListProductInstancesQueryBy::Holder,
            variant_id: None,
            status: Some(ProductInstanceStatus::Locked),
        })
        .await
        .expect("Failed to list holder instances");

    assert_eq!(locked_holder_instances.len(), 1);
    assert_eq!(locked_holder_instances[0].holder_id, alice.id);
    assert_eq!(
        locked_holder_instances[0].status,
        ProductInstanceStatus::Locked
    );

    // Complete transaction - now Bob owns and holds the item
    service
        .complete_transaction(CompleteTransactionRequest {
            transaction_id: transaction.id,
            user_id: bob.id,
        })
        .await
        .expect("Failed to complete transaction");

    // Verify: Query by owner (Bob) - should find the instance
    let bob_owner_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: bob.id,
            query_by: ListProductInstancesQueryBy::Owner,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list owner instances");

    assert_eq!(bob_owner_instances.len(), 1);
    assert_eq!(bob_owner_instances[0].owner_id, bob.id);
    assert_eq!(bob_owner_instances[0].holder_id, bob.id);

    // Verify: Query by holder (Bob) - should find the instance
    let bob_holder_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: bob.id,
            query_by: ListProductInstancesQueryBy::Holder,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list holder instances");

    assert_eq!(bob_holder_instances.len(), 1);
    assert_eq!(bob_holder_instances[0].holder_id, bob.id);
    assert_eq!(bob_holder_instances[0].owner_id, bob.id);

    // Verify: Query by holder (Alice) - should NOT find the instance anymore
    let alice_holder_instances = service
        .list_product_instances(ListProductInstancesRequest {
            user_id: alice.id,
            query_by: ListProductInstancesQueryBy::Holder,
            variant_id: None,
            status: None,
        })
        .await
        .expect("Failed to list holder instances");

    assert_eq!(alice_holder_instances.len(), 0);
}
