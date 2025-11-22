use sawa_infra_postgres::sync_schema;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let db = Database::connect(
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL environment variable must be set for integration tests"),
    )
    .await
    .expect("Failed to connect to test database");

    sync_schema(&db)
        .await
        .expect("Failed to synchronize database schema");
}
