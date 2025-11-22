mod common;

use common::{create_service, create_test_product_instance, create_user};
use sawa_core::models::misc::NonEmptyString;
use sawa_core::models::product::ProductInstanceStatus;
use sawa_core::repositories::*;
use sawa_core::services::*;

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
    let instance = create_test_product_instance(
        variant.id,
        alice.id,
        alice.id,
        ProductInstanceStatus::Active,
    );
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
