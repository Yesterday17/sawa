use aide::axum::IntoApiResponse;
use axum::{Json, http::StatusCode};
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
