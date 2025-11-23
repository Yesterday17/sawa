use std::vec;

use aide::{OperationOutput, openapi::Response};
use axum::{
    Json,
    response::{IntoResponse, Response as AxumResponse},
};
use serde_json::json;

pub enum AppError {
    Unauthorized,

    InternalServerError,
    NotFound,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> AxumResponse {
        let (status, message) = match self {
            AppError::Unauthorized => (
                axum::http::StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
            ),
            AppError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()),
            AppError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl OperationOutput for AppError {
    type Inner = Json<serde_json::Value>;

    fn operation_response(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<Response> {
        let json_schema = schemars::json_schema!({
            "type": "object",
            "properties": {
                "error": { "type": "string" }
            },
            "required": ["error"]
        });

        Some(Response {
            description: "Error response".to_string(),
            content: [(
                "application/json".to_string(),
                aide::openapi::MediaType {
                    schema: Some(aide::openapi::SchemaObject {
                        json_schema,
                        external_docs: None,
                        example: None,
                    }),
                    ..Default::default()
                },
            )]
            .into(),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<aide::openapi::StatusCode>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            vec![
                (Some(aide::openapi::StatusCode::Code(400)), res.clone()),
                (Some(aide::openapi::StatusCode::Code(404)), res.clone()),
                (Some(aide::openapi::StatusCode::Code(500)), res),
            ]
        } else {
            vec![]
        }
    }
}
