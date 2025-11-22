//! Integration tests for PostgreSQL repository implementations
//!
//! These tests require a PostgreSQL database to be running.
//! Set DATABASE_URL environment variable to configure the connection.
//!
//! Example:
//!   DATABASE_URL=postgresql://user:password@localhost:5432/test_db cargo test

use sawa_infra_postgres::*;
use sawa_repository_tests::test_all_repositories;
use sea_orm::Database;

/// Helper function to create a database connection for testing
async fn create_test_db() -> sea_orm::DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set for integration tests");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    db
}

test_all_repositories! {
    product => PostgresProductRepository::new(create_test_db().await),
    product_variant => PostgresProductVariantRepository::new(create_test_db().await),
    product_instance => PostgresProductInstanceRepository::new(create_test_db().await),
    purchase_order => PostgresPurchaseOrderRepository::new(create_test_db().await),
    user => PostgresUserRepository::new(create_test_db().await),
    user_transaction => PostgresUserTransactionRepository::new(create_test_db().await),
    media => PostgresMediaRepository::new(create_test_db().await),
    tag => PostgresTagRepository::new(create_test_db().await),
}
