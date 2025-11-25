use crate::{error::AppError, state::AppState};
use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{Json, extract::State, http::StatusCode};
use sawa_core::{
    models::misc::{Tag, TagId},
    services::{LoadTagsRequest, TagService},
};

/// POST /tags/batch
pub async fn get_tags_batch<S>(
    State(state): State<AppState<S>>,
    Json(tag_ids): Json<Vec<TagId>>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: TagService,
{
    let req = LoadTagsRequest { ids: tag_ids };
    let tags = state
        .service
        .load_tags(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(tags)))
}

pub fn create_get_tags_batch_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get tags in batch")
        .description("Get multiple tags by their IDs.")
        .tag("Tag")
        .response::<200, Json<Vec<Tag>>>()
}
