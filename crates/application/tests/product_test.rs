mod common;

use common::create_service;
use sawa_core::models::misc::{Currency, NonEmptyString, Price};
use sawa_core::services::*;

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
