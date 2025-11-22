use aide::{
    openapi::Response,
    OperationOutput,
};
use axum::{
    response::{IntoResponse, Response as AxumResponse},
    Json,
};
use serde_json::json;

pub enum AppError {
    InternalServerError,
    NotFound,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> AxumResponse {
        let (status, message) = match self {
            AppError::InternalServerError => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, "Not Found".to_string()),
            AppError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl OperationOutput for AppError {
    type Inner = Json<serde_json::Value>;

    fn operation_response(ctx: &mut aide::r#gen::GenContext, _operation: &mut aide::openapi::Operation) -> Option<Response> {
        let mut schema = schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Object.into()),
            ..Default::default()
        };
        schema.object().required.insert("error".to_string());
        schema.object().properties.insert("error".to_string(), <String as schemars::JsonSchema>::json_schema(&mut ctx.schema));
        
        Some(Response {
            description: "Error response".to_string(),
            content: [("application/json".to_string(), aide::openapi::MediaType {
                schema: Some(aide::openapi::SchemaObject {
                    json_schema: schemars::schema::Schema::Object(schema),
                    external_docs: None,
                    example: None,
                }),
                ..Default::default()
            })].into(),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut aide::r#gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            vec![
                (Some(400), res.clone()),
                (Some(404), res.clone()),
                (Some(500), res),
            ]
        } else {
            vec![]
        }
    }
}
