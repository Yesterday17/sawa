use aide::axum::IntoApiResponse;
use axum::{http::StatusCode, Json};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct HealthResponse {
    status: String,
}

pub async fn health_check() -> impl IntoApiResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok".to_string(),
        }),
    )
}
