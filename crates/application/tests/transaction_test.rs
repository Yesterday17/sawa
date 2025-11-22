mod common;

use common::{create_service, create_test_product_instance, create_user};
use sawa_core::models::misc::NonEmptyString;
use sawa_core::models::product::ProductInstanceStatus;
use sawa_core::repositories::*;
use sawa_core::services::*;

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
    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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
    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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
    let instance1 = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
    let instance2 = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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

    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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

    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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

    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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

    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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

    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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

    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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
        let instance2 = create_test_product_instance(
            variant.id,
            alice.id,
            alice.id,
            ProductInstanceStatus::Active,
        );
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
    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Consumed,
    );
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
    let instance =
        create_test_product_instance(variant.id, bob.id, bob.id, ProductInstanceStatus::Active);
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
