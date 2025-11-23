use sawa_api::create_app;
use sawa_application::Service;
use sawa_infra_memory::{
    InMemoryMediaRepository, InMemoryProductInstanceRepository, InMemoryProductRepository,
    InMemoryProductVariantRepository, InMemoryPurchaseOrderRepository, InMemoryTagRepository,
    InMemoryUserRepository, InMemoryUserTransactionRepository,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_sessions_memory_store::MemoryStore;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create repositories
    let product = InMemoryProductRepository::new();
    let product_variant = InMemoryProductVariantRepository::new();
    let product_instance = InMemoryProductInstanceRepository::new();
    let order = InMemoryPurchaseOrderRepository::new();
    let transaction = InMemoryUserTransactionRepository::new();
    let user = InMemoryUserRepository::new();
    let tag = InMemoryTagRepository::new();
    let media = InMemoryMediaRepository::new();

    // Create service
    let service = Service {
        product,
        product_variant,
        product_instance,
        order,
        transaction,
        user,
        tag,
        media,
    };

    // Create the app
    let session_store = MemoryStore::default();
    let app = create_app(service, session_store).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .into_inner(),
    );

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server listening on http://{}", addr);
    println!("Docs available at http://{}/scalar", addr);

    // Start the server
    axum::serve(listener, app).await.unwrap();
}
