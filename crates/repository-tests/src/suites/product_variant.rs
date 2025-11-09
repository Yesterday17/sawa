use sawa_core::{
    models::{
        misc::{NonEmptyString, TagId},
        product::{ProductId, ProductVariant},
    },
    repositories::ProductVariantRepository,
};

fn make_string(s: &str) -> NonEmptyString {
    unsafe { NonEmptyString::new_unchecked(s.to_string()) }
}

fn create_test_variant(product_id: ProductId, name: &str) -> ProductVariant {
    ProductVariant::new(product_id, make_string(name))
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: ProductVariantRepository>(repo: R) {
    let variant = create_test_variant(ProductId::new(), "Test Variant");
    let variant_id = variant.id;

    repo.save(&variant).await.unwrap();

    let found = repo.find_by_id(&variant_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, variant_id);
}

/// Test find_by_product_id returns variants for that product.
pub async fn test_find_by_product_id<R: ProductVariantRepository>(repo: R) {
    let product_id = ProductId::new();
    let variant1 = create_test_variant(product_id, "Variant 1");
    let variant2 = create_test_variant(product_id, "Variant 2");
    let other_variant = create_test_variant(ProductId::new(), "Other");

    repo.save(&variant1).await.unwrap();
    repo.save(&variant2).await.unwrap();
    repo.save(&other_variant).await.unwrap();

    let variants = repo.find_by_product_id(&product_id).await.unwrap();
    assert_eq!(variants.len(), 2);
    assert!(variants.iter().any(|v| v.id == variant1.id));
    assert!(variants.iter().any(|v| v.id == variant2.id));
}

/// Test find_by_tags_all returns variants with all tags.
pub async fn test_find_by_tags_all<R: ProductVariantRepository>(repo: R) {
    let tag1 = TagId::new();
    let tag2 = TagId::new();

    let mut variant_both = create_test_variant(ProductId::new(), "Both Tags");
    variant_both.add_tag(tag1);
    variant_both.add_tag(tag2);

    let mut variant_one = create_test_variant(ProductId::new(), "One Tag");
    variant_one.add_tag(tag1);

    repo.save(&variant_both).await.unwrap();
    repo.save(&variant_one).await.unwrap();

    // Query for both tags - should only return variant_both
    let results = repo.find_by_tags_all(&[tag1, tag2]).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, variant_both.id);
}

/// Test find_by_tags_any returns variants with any of the tags.
pub async fn test_find_by_tags_any<R: ProductVariantRepository>(repo: R) {
    let tag1 = TagId::new();
    let tag2 = TagId::new();
    let tag3 = TagId::new();

    let mut variant1 = create_test_variant(ProductId::new(), "Tag 1");
    variant1.add_tag(tag1);

    let mut variant2 = create_test_variant(ProductId::new(), "Tag 2");
    variant2.add_tag(tag2);

    let mut variant3 = create_test_variant(ProductId::new(), "Tag 3");
    variant3.add_tag(tag3);

    repo.save(&variant1).await.unwrap();
    repo.save(&variant2).await.unwrap();
    repo.save(&variant3).await.unwrap();

    // Query for tag1 or tag2 - should return variant1 and variant2
    let results = repo.find_by_tags_any(&[tag1, tag2]).await.unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|v| v.id == variant1.id));
    assert!(results.iter().any(|v| v.id == variant2.id));
}

/// Test delete removes variant.
pub async fn test_delete<R: ProductVariantRepository>(repo: R) {
    let variant = create_test_variant(ProductId::new(), "To Delete");
    let variant_id = variant.id;

    repo.save(&variant).await.unwrap();
    repo.delete(&variant_id).await.unwrap();

    let after_delete = repo.find_by_id(&variant_id).await.unwrap();
    assert!(after_delete.is_none());
}
