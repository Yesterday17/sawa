//! Contract tests for in-memory repository implementations

use sawa_infra_memory::*;
use sawa_repository_tests::test_all_repositories;

test_all_repositories! {
    product => InMemoryProductRepository::new(),
    product_variant => InMemoryProductVariantRepository::new(),
    product_instance => InMemoryProductInstanceRepository::new(),
    purchase_order => InMemoryPurchaseOrderRepository::new(),
    user => InMemoryUserRepository::new(),
    user_transaction => InMemoryUserTransactionRepository::new(),
    media => InMemoryMediaRepository::new(),
    tag => InMemoryTagRepository::new(),
}
