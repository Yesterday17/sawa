use crate::{auth::AuthSession, error::AppError, state::AppState};
use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use axum_login::AuthUser;
use sawa_core::{
    models::product::{ProductInstance, ProductInstanceStatus, ProductVariantId},
    services::{
        ListProductInstancesQueryBy, ListProductInstancesRequest, ProductInstanceService,
        UserService,
    },
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct QueryByPath {
    pub query_by: ListProductInstancesQueryBy,
}

#[derive(Deserialize, JsonSchema)]
pub struct ListProductInstanceQuery {
    pub status: Option<ProductInstanceStatus>,
    pub variant_id: Option<ProductVariantId>,
}

/// GET /goods/{query_by}
pub async fn list_product_instances<S>(
    State(state): State<AppState<S>>,
    Path(QueryByPath { query_by }): Path<QueryByPath>,
    Query(ListProductInstanceQuery { status, variant_id }): Query<ListProductInstanceQuery>,
    auth_session: AuthSession<S>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductInstanceService + UserService + Clone,
{
    let user = auth_session.user.as_ref().ok_or(AppError::Unauthorized)?;

    let req = ListProductInstancesRequest {
        user_id: user.id(),
        query_by,
        variant_id,
        status,
    };

    let instances = state
        .service
        .list_product_instances(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(instances)))
}

pub fn create_list_product_instances_docs(op: TransformOperation) -> TransformOperation {
    op.summary("List goods")
        .description("List product instances owned or held by a user.")
        .tag("Goods")
        .response::<200, Json<Vec<ProductInstance>>>()
}
