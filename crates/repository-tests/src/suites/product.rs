use sawa_core::{
    models::{
        misc::NonEmptyString,
        product::{Product, ProductId},
    },
    repositories::ProductRepository,
};

// Helper for creating test NonEmptyString
fn make_string(s: &str) -> NonEmptyString {
    unsafe { NonEmptyString::new_unchecked(s.to_string()) }
}

/// Test find_by_id returns None for non-existent ID.
pub async fn test_find_by_id_returns_none_for_non_existent<R: ProductRepository>(repo: R) {
    let non_existent_id = ProductId::new();
    let result = repo.find_by_id(&non_existent_id).await.unwrap();
    assert!(result.is_none());
}

/// Test save and find_by_id.
pub async fn test_save_and_find_by_id<R: ProductRepository>(repo: R) {
    let product = Product::new(make_string("Test Product"), "Test description".to_string());
    let product_id = product.id;

    repo.save(&product).await.unwrap();

    let found = repo.find_by_id(&product_id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, product_id);
}

/// Test find_all includes saved products.
pub async fn test_find_all<R: ProductRepository>(repo: R) {
    let product1 = Product::new(make_string("Product 1"), "Description 1".to_string());
    let product2 = Product::new(make_string("Product 2"), "Description 2".to_string());

    repo.save(&product1).await.unwrap();
    repo.save(&product2).await.unwrap();

    let all = repo.find_all().await.unwrap();
    assert!(all.len() >= 2);
    assert!(all.iter().any(|p| p.id == product1.id));
    assert!(all.iter().any(|p| p.id == product2.id));
}

/// Test delete removes product.
pub async fn test_delete<R: ProductRepository>(repo: R) {
    let product = Product::new(make_string("To Delete"), "Will be deleted".to_string());
    let product_id = product.id;

    repo.save(&product).await.unwrap();
    repo.delete(&product_id).await.unwrap();

    let after_delete = repo.find_by_id(&product_id).await.unwrap();
    assert!(after_delete.is_none());
}
