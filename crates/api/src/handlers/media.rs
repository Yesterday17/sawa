use crate::{error::AppError, state::AppState};
use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use sawa_core::{
    models::misc::{Media, MediaId},
    services::{CreateMediaBatchRequest, GetMediaRequest, MediaService},
};
use schemars::JsonSchema;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, JsonSchema)]
pub struct CreateMediaBatchBody {
    pub urls: Vec<Url>,
}

/// POST /media/batch
pub async fn create_media_batch<S>(
    State(state): State<AppState<S>>,
    Json(body): Json<CreateMediaBatchBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: MediaService,
{
    let req = CreateMediaBatchRequest { urls: body.urls };

    let medias = state
        .service
        .create_media_batch(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(medias)))
}

pub fn create_create_media_batch_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create media batch")
        .description("Create multiple media entries from URLs.")
        .tag("Media")
        .response::<201, Json<Vec<Media>>>()
}

#[derive(Deserialize, JsonSchema)]
pub struct MediaIdPath {
    pub media_id: MediaId,
}

/// GET /media/{media_id}
pub async fn get_media<S>(
    State(state): State<AppState<S>>,
    Path(MediaIdPath { media_id }): Path<MediaIdPath>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: MediaService,
{
    let req = GetMediaRequest { id: media_id };

    let media = state
        .service
        .get_media(req)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(Redirect::temporary(media.url.as_str()))
}

pub fn create_get_media_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get media")
        .description("Get a media item by its ID and redirect to the actual URL.")
        .tag("Media")
        .response::<307, ()>()
}
