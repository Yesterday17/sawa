use chrono::Utc;
use sawa_application::Service;
use sawa_core::models::misc::NonEmptyString;
use sawa_core::models::product::{ProductInstance, ProductInstanceId, ProductInstanceStatus};
use sawa_core::models::purchase::PurchaseOrderLineItemId;
use sawa_core::models::user::{Email, User, UserId, Username};
use sawa_infra_memory::*;

pub type TestService = Service<
    InMemoryProductRepository,
    InMemoryProductVariantRepository,
    InMemoryProductInstanceRepository,
    InMemoryPurchaseOrderRepository,
    InMemoryUserTransactionRepository,
    InMemoryUserRepository,
    InMemoryTagRepository,
    InMemoryMediaRepository,
>;

pub fn create_service() -> TestService {
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

#[allow(dead_code)]
pub fn create_user(username: &str) -> User {
    User {
        id: UserId::new(),
        username: Username(username.to_string()),
        email: Email(format!("{}@example.com", username)),
        password_hash: NonEmptyString::new("hash".to_string()).unwrap(),
        avatar: None,
        created_at: Utc::now(),
    }
}

#[allow(dead_code)]
pub fn create_test_product_instance(
    variant_id: sawa_core::models::product::ProductVariantId,
    owner_id: UserId,
    holder_id: UserId,
    status: ProductInstanceStatus,
) -> ProductInstance {
    ProductInstance {
        id: ProductInstanceId::new(),
        variant_id,
        owner_id,
        holder_id,
        status,
        source_order_line_item_id: PurchaseOrderLineItemId::new(),
        created_at: Utc::now(),
        transfer_history: vec![],
        status_history: vec![],
    }
}
