use aide::{
    axum::{
        ApiRouter,
        routing::{get, post},
    },
    openapi::OpenApi,
};
use axum::Router;
use sawa_core::services::ProductService;
use state::AppState;

pub mod docs;
pub mod error;
pub mod handlers;
pub mod state;

pub fn create_app<S>(state: S) -> Router
where
    S: Clone + Send + Sync + 'static + ProductService,
{
    let mut api = OpenApi::default();

    // Define the API routes
    let api_router = ApiRouter::new()
        .api_route("/health", get(handlers::health::health_check))
        .api_route(
            "/products",
            post(handlers::product::create_product::<S>).get(handlers::product::list_products::<S>),
        )
        .api_route(
            "/products/:product_id",
            get(handlers::product::get_product::<S>),
        )
        .api_route(
            "/products/variants",
            get(handlers::product::list_product_variants::<S>),
        )
        .api_route(
            "/products/:product_id/variants",
            post(handlers::product::create_product_variant::<S>)
                .get(handlers::product::list_product_variants::<S>),
        )
        .api_route(
            "/products/:product_id/variants/:variant_id",
            get(handlers::product::get_product_variant::<S>),
        )
        .with_state(AppState::new(state));

    // Finish the API to populate `api`
    let app = api_router.finish_api_with(&mut api, |api| api.title("Sawa API").version("0.1.0"));

    // Create docs router which serves the `api`
    let docs_router = docs::docs_routes(api);

    // Merge them and convert to axum Router
    app.merge(docs_router).into()
}
