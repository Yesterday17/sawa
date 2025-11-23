use crate::auth::AuthBackend;
use aide::{
    axum::{
        ApiRouter,
        routing::{get, get_with, post_with},
    },
    openapi::OpenApi,
};
use axum::{Router, response::IntoResponse};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{Expiry, SessionManagerLayer, SessionStore},
};
use sawa_core::services::{ProductService, PurchaseOrderService, UserService};
use state::AppState;

pub mod auth;
pub mod docs;
pub mod error;
pub mod handlers;
pub mod state;

macro_rules! ensure_login {
    () => {{
        axum::middleware::from_fn(
            |auth_session: crate::auth::AuthSession<S>, req, next: axum::middleware::Next| async move {
                if auth_session.user.is_some() {
                    next.run(req).await
                } else {
                    axum::http::StatusCode::UNAUTHORIZED.into_response()
                }
            },
        )
    }};
}

pub fn create_app<S, SS>(state: S, session_store: SS) -> Router
where
    S: Clone + ProductService + UserService + PurchaseOrderService,
    SS: Clone + SessionStore,
{
    let mut api = OpenApi::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_name("sawa.id")
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)));
    let auth_backend = AuthBackend::new(state.clone());
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    // Define the API routes
    let api_router = ApiRouter::new()
        .api_route("/health", get(handlers::health::health_check))
        .api_route(
            "/user/login",
            post_with(
                handlers::auth::login::<S>,
                handlers::auth::create_login_docs,
            ),
        )
        .api_route(
            "/user/logout",
            post_with(
                handlers::auth::logout::<S>,
                handlers::auth::create_logout_docs,
            ),
        )
        .api_route(
            "/user/register",
            post_with(
                handlers::auth::register::<S>,
                handlers::auth::create_register_docs,
            ),
        )
        .api_route(
            "/products",
            post_with(
                handlers::product::create_product::<S>,
                handlers::product::create_create_product_docs,
            )
            .route_layer(ensure_login!())
            .get_with(
                handlers::product::list_products::<S>,
                handlers::product::create_list_products_docs,
            ),
        )
        .api_route(
            "/products/{product_id}",
            get_with(
                handlers::product::get_product::<S>,
                handlers::product::create_get_product_docs,
            ),
        )
        .api_route(
            "/products/variants",
            get_with(
                handlers::product::list_product_variants::<S>,
                handlers::product::create_list_product_variants_docs,
            ),
        )
        .api_route(
            "/products/{product_id}/variants",
            post_with(
                handlers::product::create_product_variant::<S>,
                handlers::product::create_create_product_variant_docs,
            )
            .route_layer(ensure_login!())
            .get_with(
                handlers::product::list_product_variants::<S>,
                handlers::product::create_list_product_variants_docs,
            ),
        )
        .api_route(
            "/products/{product_id}/variants/{variant_id}",
            get_with(
                handlers::product::get_product_variant::<S>,
                handlers::product::create_get_product_variant_docs,
            ),
        )
        .api_route(
            "/orders",
            post_with(
                handlers::purchase_order::create_order::<S>,
                handlers::purchase_order::create_create_order_docs,
            )
            .route_layer(ensure_login!())
            .get_with(
                handlers::purchase_order::list_orders::<S>,
                handlers::purchase_order::create_list_orders_docs,
            )
            .route_layer(ensure_login!()),
        )
        .layer(auth_layer)
        .with_state(AppState::new(state));

    // Finish the API to populate `api`
    let app = api_router.finish_api_with(&mut api, |api| api.title("Sawa API").version("0.1.0"));

    // Create docs router which serves the `api`
    let docs_router = docs::docs_routes(api);

    // Merge them and convert to axum Router
    app.merge(docs_router).into()
}
