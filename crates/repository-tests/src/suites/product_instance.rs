use sawa_core::{
    models::{
        product::{ProductInstance, ProductInstanceId, ProductInstanceStatus, ProductVariantId},
        purchase::PurchaseOrderLineItemId,
        user::UserId,
    },
    repositories::ProductInstanceRepository,
};

// TODO: Owner and holder should be different
fn create_test_instance(owner_id: UserId, variant_id: ProductVariantId) -> ProductInstance {
    ProductInstance {
        id: ProductInstanceId::new(),
        variant_id,
        owner_id,
        holder_id: owner_id,
        status: ProductInstanceStatus::Active,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: chrono::Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    }
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: ProductInstanceRepository>(repo: R) {
    let instance = create_test_instance(UserId::new(), ProductVariantId::new());
    let instance_id = instance.id;

    repo.save(&instance).await.unwrap();

    let found = repo.find_by_id(&instance_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, instance_id);
}

/// Test find_by_owner returns user's instances.
pub async fn test_find_by_owner<R: ProductInstanceRepository>(repo: R) {
    let owner_id = UserId::new();
    let instance1 = create_test_instance(owner_id, ProductVariantId::new());
    let instance2 = create_test_instance(owner_id, ProductVariantId::new());

    repo.save(&instance1).await.unwrap();
    repo.save(&instance2).await.unwrap();

    let owned = repo.find_by_owner(&owner_id).await.unwrap();
    assert_eq!(owned.len(), 2);
}

/// Test find_by_owner_and_variant filters correctly.
pub async fn test_find_by_owner_and_variant<R: ProductInstanceRepository>(repo: R) {
    let owner_id = UserId::new();
    let variant1 = ProductVariantId::new();
    let variant2 = ProductVariantId::new();

    let instance1 = create_test_instance(owner_id, variant1);
    let instance2 = create_test_instance(owner_id, variant2);

    repo.save(&instance1).await.unwrap();
    repo.save(&instance2).await.unwrap();

    let by_variant = repo
        .find_by_owner_and_variant(&owner_id, &variant1)
        .await
        .unwrap();
    assert_eq!(by_variant.len(), 1);
    assert_eq!(by_variant[0].variant_id, variant1);
}

/// Test find_by_owner_and_status filters by status.
pub async fn test_find_by_owner_and_status<R: ProductInstanceRepository>(repo: R) {
    let owner_id = UserId::new();
    let active_instance = create_test_instance(owner_id, ProductVariantId::new());
    let mut consumed_instance = create_test_instance(owner_id, ProductVariantId::new());
    consumed_instance.status = ProductInstanceStatus::Consumed;

    repo.save(&active_instance).await.unwrap();
    repo.save(&consumed_instance).await.unwrap();

    let active = repo
        .find_by_owner_and_status(&owner_id, ProductInstanceStatus::Active)
        .await
        .unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, active_instance.id);

    let consumed = repo
        .find_by_owner_and_status(&owner_id, ProductInstanceStatus::Consumed)
        .await
        .unwrap();
    assert_eq!(consumed.len(), 1);
    assert_eq!(consumed[0].id, consumed_instance.id);
}

/// Test delete removes instance.
pub async fn test_delete<R: ProductInstanceRepository>(repo: R) {
    let instance = create_test_instance(UserId::new(), ProductVariantId::new());
    let instance_id = instance.id;

    repo.save(&instance).await.unwrap();
    repo.delete(&instance_id).await.unwrap();

    let after_delete = repo.find_by_id(&instance_id).await.unwrap();
    assert!(after_delete.is_none());
}

/// Test find_by_owner only returns instances owned by that user (permission check).
pub async fn test_find_by_owner_permission_isolation<R: ProductInstanceRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();

    let instance_a1 = create_test_instance(user_a, ProductVariantId::new());
    let instance_a2 = create_test_instance(user_a, ProductVariantId::new());
    let instance_b = create_test_instance(user_b, ProductVariantId::new());

    repo.save(&instance_a1).await.unwrap();
    repo.save(&instance_a2).await.unwrap();
    repo.save(&instance_b).await.unwrap();

    // User A should only see their own instances
    let user_a_instances = repo.find_by_owner(&user_a).await.unwrap();
    assert_eq!(user_a_instances.len(), 2);
    assert!(user_a_instances.iter().all(|i| i.owner_id == user_a));
    assert!(user_a_instances.iter().all(|i| i.owner_id != user_b));

    // User B should only see their own instance
    let user_b_instances = repo.find_by_owner(&user_b).await.unwrap();
    assert_eq!(user_b_instances.len(), 1);
    assert_eq!(user_b_instances[0].owner_id, user_b);
}

/// Test find_by_owner_and_variant respects owner permission.
pub async fn test_find_by_owner_and_variant_permission<R: ProductInstanceRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();
    let variant = ProductVariantId::new();

    let instance_a = create_test_instance(user_a, variant);
    let instance_b = create_test_instance(user_b, variant);

    repo.save(&instance_a).await.unwrap();
    repo.save(&instance_b).await.unwrap();

    // User A queries for the variant - should only see their own
    let results = repo
        .find_by_owner_and_variant(&user_a, &variant)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].owner_id, user_a);

    // User B queries for the same variant - should only see theirs
    let results = repo
        .find_by_owner_and_variant(&user_b, &variant)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].owner_id, user_b);
}

/// Test find_by_owner_and_status respects owner permission.
pub async fn test_find_by_owner_and_status_permission<R: ProductInstanceRepository>(repo: R) {
    let user_a = UserId::new();
    let user_b = UserId::new();

    let instance_a = create_test_instance(user_a, ProductVariantId::new());
    let instance_b = create_test_instance(user_b, ProductVariantId::new());

    repo.save(&instance_a).await.unwrap();
    repo.save(&instance_b).await.unwrap();

    // User A queries for Active status - should only see their own
    let results = repo
        .find_by_owner_and_status(&user_a, ProductInstanceStatus::Active)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].owner_id, user_a);
    assert!(results.iter().all(|i| i.owner_id != user_b));
}
