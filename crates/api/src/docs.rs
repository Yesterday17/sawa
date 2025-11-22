use aide::{
    axum::{
        routing::get,
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
    scalar::Scalar,
};
use axum::{Extension, Json};
use std::sync::Arc;

pub fn docs_routes(api: OpenApi) -> ApiRouter {
    ApiRouter::new()
        .route("/redoc", Redoc::new("/docs/private/api.json").axum_route())
        .route("/scalar", Scalar::new("/docs/private/api.json").axum_route())
        .api_route("/docs/private/api.json", get(serve_docs))
        .layer(Extension(Arc::new(api)))
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api)
}
