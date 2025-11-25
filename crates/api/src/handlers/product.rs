use crate::{error::AppError, state::AppState};
use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sawa_core::{
    models::{
        misc::{MediaId, NonEmptyString, Price, TagId},
        product::{MysteryBoxConfig, Product, ProductId, ProductVariant, ProductVariantId},
    },
    services::{
        CreateProductRequest, CreateProductVariantRequest, GetProductRequest,
        GetProductVariantRequest, ListProductVariantsRequest, ListProductsRequest,
        LoadProductVariantsRequest, ProductService, TagMatchPolicy,
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

pub fn create_list_products_docs(op: TransformOperation) -> TransformOperation {
    op.summary("List products")
        .description("List all products.")
        .tag("Product")
        .response::<200, Json<Vec<Product>>>()
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

pub fn create_create_product_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create product")
        .description("Create a new product.")
        .tag("Product")
        .response::<201, Json<Product>>()
}

#[derive(Deserialize, JsonSchema)]
pub struct ProductIdPath {
    pub product_id: ProductId,
}

/// GET /products/{product_id}
pub async fn get_product<S>(
    State(state): State<AppState<S>>,
    Path(ProductIdPath { product_id }): Path<ProductIdPath>,
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

pub fn create_get_product_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get product")
        .description("Get a product by its ID.")
        .tag("Product")
        .response::<200, Json<Product>>()
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

#[derive(Deserialize, JsonSchema)]
pub struct ListProductVariantsQuery {
    pub tags: Option<Vec<TagId>>,
    pub tag_match: Option<TagMatchPolicy>,
}

/// GET /products/variants
pub async fn list_all_product_variants<S>(
    State(state): State<AppState<S>>,
    Query(query): Query<ListProductVariantsQuery>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = ListProductVariantsRequest {
        product_id: None,
        tags: query.tags,
        tag_match_policy: query.tag_match.unwrap_or(TagMatchPolicy::Any),
    };
    let variants = state
        .service
        .list_product_variants(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(variants)))
}

pub fn create_list_all_product_variants_docs(op: TransformOperation) -> TransformOperation {
    op.summary("List all product variants")
        .description("List all product variants.")
        .tag("Product Variant")
        .response::<200, Json<Vec<ProductVariant>>>()
}

/// GET /products/{product_id}/variants
pub async fn list_product_variants<S>(
    Path(ProductIdPath { product_id }): Path<ProductIdPath>,
    State(state): State<AppState<S>>,
    Query(query): Query<ListProductVariantsQuery>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = ListProductVariantsRequest {
        product_id: Some(product_id),
        tags: query.tags,
        tag_match_policy: query.tag_match.unwrap_or(TagMatchPolicy::Any),
    };
    let variants = state
        .service
        .list_product_variants(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(variants)))
}

pub fn create_list_product_variants_docs(op: TransformOperation) -> TransformOperation {
    op.summary("List product variants")
        .description("List product variants, optionally filtered by product ID.")
        .tag("Product Variant")
        .response::<200, Json<Vec<ProductVariant>>>()
}

/// POST /products/{product_id}/variants
pub async fn create_product_variant<S>(
    State(state): State<AppState<S>>,
    Path(ProductIdPath { product_id }): Path<ProductIdPath>,
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

pub fn create_create_product_variant_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create product variant")
        .description("Create a new product variant for a specific product.")
        .tag("Product Variant")
        .response::<201, Json<ProductVariant>>()
}

#[derive(Deserialize, JsonSchema)]
pub struct ProductIdVariantIdPath {
    pub product_id: ProductId,
    pub variant_id: ProductVariantId,
}

/// GET /products/{product_id}/variants/{variant_id}
pub async fn get_product_variant<S>(
    State(state): State<AppState<S>>,
    Path(ProductIdVariantIdPath {
        product_id: _,
        variant_id,
    }): Path<ProductIdVariantIdPath>,
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

pub fn create_get_product_variant_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get product variant")
        .description("Get a product variant by its ID.")
        .tag("Product Variant")
        .response::<200, Json<ProductVariant>>()
}

/// POST /products/{product_id}/variants/batch
pub async fn load_product_variants<S>(
    State(state): State<AppState<S>>,
    Json(variant_ids): Json<Vec<ProductVariantId>>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: ProductService,
{
    let req = LoadProductVariantsRequest { ids: variant_ids };
    let variants = state
        .service
        .load_product_variants(req)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok((StatusCode::OK, Json(variants)))
}

pub fn create_load_product_variants_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Load product variants in batch")
        .description("Load multiple product variants by their IDs.")
        .tag("Product Variant")
        .response::<200, Json<Vec<ProductVariant>>>()
}
