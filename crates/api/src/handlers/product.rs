use crate::{error::AppError, state::AppState};
use aide::axum::IntoApiResponse;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sawa_core::{
    models::{
        misc::{MediaId, NonEmptyString, Price},
        product::{MysteryBoxConfig, ProductId, ProductVariantId},
    },
    services::{
        CreateProductRequest, CreateProductVariantRequest, GetProductRequest,
        GetProductVariantRequest, ListProductVariantsRequest, ListProductsRequest, ProductService,
    },
};
use schemars::JsonSchema;
use serde::Deserialize;

/// GET /products
pub async fn list_products<S>(
    State(state): State<AppState<S>>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = ListProductsRequest {};
    let products = state
        .service
        .list_products(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(products)))
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateProductBody {
    pub name: NonEmptyString,
    pub description: String,
    pub medias: Vec<MediaId>,
}

/// POST /products
pub async fn create_product<S>(
    State(state): State<AppState<S>>,
    Json(body): Json<CreateProductBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = CreateProductRequest {
        name: body.name,
        description: body.description,
        medias: body.medias,
    };

    let product = state
        .service
        .create_product(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(product)))
}

/// GET /products/{product_id}
pub async fn get_product<S>(
    State(state): State<AppState<S>>,
    Path(product_id): Path<ProductId>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = GetProductRequest { id: product_id };

    let product = state
        .service
        .get_product(req)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok((StatusCode::OK, Json(product)))
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateProductVariantBody {
    pub name: NonEmptyString,
    pub description: String,
    pub medias: Vec<MediaId>,
    pub tags: Vec<NonEmptyString>,
    pub price: Option<Price>,
    pub mystery_box: Option<MysteryBoxConfig>,
    pub sort_order: i32,
}

/// GET /products/{product_id}/variants
/// GET /products/variants
pub async fn list_product_variants<S>(
    Path(product_id): Path<Option<ProductId>>,
    State(state): State<AppState<S>>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = ListProductVariantsRequest {
        product_id,
        tags: None,
        match_all_tags: false,
    };
    let variants = state
        .service
        .list_product_variants(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(variants)))
}

/// POST /products/{product_id}/variants
pub async fn create_product_variant<S>(
    State(state): State<AppState<S>>,
    Path(product_id): Path<ProductId>,
    Json(body): Json<CreateProductVariantBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = CreateProductVariantRequest {
        product_id,
        name: body.name,
        description: body.description,
        medias: body
            .medias
            .into_iter()
            .map(|id| MediaId::try_from(id).unwrap())
            .collect(),
        tags: body.tags,
        price: body.price,
        mystery_box: body.mystery_box,
        sort_order: body.sort_order,
    };

    let variant = state
        .service
        .create_product_variant(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(variant)))
}

#[derive(Deserialize, JsonSchema)]
pub struct GetProductVariantPath {
    pub product_id: ProductId,
    pub variant_id: ProductVariantId,
}

/// GET /products/{product_id}/variants/{variant_id}
pub async fn get_product_variant<S>(
    State(state): State<AppState<S>>,
    Path(GetProductVariantPath {
        product_id: _,
        variant_id,
    }): Path<GetProductVariantPath>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = GetProductVariantRequest { id: variant_id };

    let variant = state
        .service
        .get_product_variant(req)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok((StatusCode::OK, Json(variant)))
}
