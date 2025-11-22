use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get},
    openapi::OpenApi,
    scalar::Scalar,
};
use axum::{Extension, Json};
use std::sync::Arc;

pub fn docs_routes(api: OpenApi) -> ApiRouter {
    ApiRouter::new()
        .route(
            "/scalar",
            Scalar::new("/docs/private/api.json").axum_route(),
        )
        .api_route("/docs/private/api.json", get(serve_docs))
        .layer(Extension(Arc::new(api)))
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api)
}
