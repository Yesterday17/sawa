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
            owner_id: user.id,
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
