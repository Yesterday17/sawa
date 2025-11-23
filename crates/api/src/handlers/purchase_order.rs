use crate::{auth::AuthSession, error::AppError, state::AppState};
use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use axum_login::AuthUser;
use sawa_core::{
    models::{
        misc::{Address, Price},
        product::ProductVariantId,
        purchase::{
            OrderRoleFilter, PurchaseOrder, PurchaseOrderId, PurchaseOrderItemId,
            PurchaseOrderStatus,
        },
        user::UserId,
    },
    services::{
        AddOrderItemRequest, CreateOrderItemRequest, CreateOrderRequest, GetOrderRequest,
        ListOrdersRequest, PurchaseOrderService, SubmitMysteryBoxResultsRequest, UserService,
    },
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::num::NonZeroU32;

#[derive(Deserialize, JsonSchema)]
pub struct CreateOrderBody {
    pub receiver_id: Option<UserId>,
    pub shipping_address: Option<Address>,
    pub total_price: Option<Price>,
    pub items: Vec<CreateOrderItemBody>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateOrderItemBody {
    pub variant_id: ProductVariantId,
    pub owner_id: Option<UserId>,
    pub quantity: NonZeroU32,
    pub unit_price: Option<Price>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AddOrderItemBody {
    pub variant_id: ProductVariantId,
    pub owner_id: UserId,
    pub quantity: NonZeroU32,
    pub unit_price: Option<Price>,
}

#[derive(Deserialize, JsonSchema)]
pub struct SubmitMysteryBoxResultsBody {
    pub owner_id: UserId,
    pub received_variants: Vec<ProductVariantId>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ListOrdersQuery {
    pub role: OrderRoleFilter,
    pub status: Option<PurchaseOrderStatus>,
}

/// POST /orders
pub async fn create_order<S>(
    State(state): State<AppState<S>>,
    auth_session: AuthSession<S>,
    Json(body): Json<CreateOrderBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: PurchaseOrderService + UserService + Clone,
{
    let user = auth_session.user.as_ref().ok_or(AppError::Unauthorized)?;

    let req = CreateOrderRequest {
        user_id: user.id(),
        receiver_id: body.receiver_id,
        shipping_address: body.shipping_address,
        total_price: body.total_price,
        items: body
            .items
            .into_iter()
            .map(|item| CreateOrderItemRequest {
                variant_id: item.variant_id,
                owner_id: item.owner_id,
                quantity: item.quantity,
                unit_price: item.unit_price,
            })
            .collect(),
    };

    let order = state
        .service
        .create_order(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(order)))
}

pub fn create_create_order_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create order")
        .description("Create a new purchase order.")
        .tag("Purchase Order")
        .response::<201, Json<PurchaseOrder>>()
}

/// GET /orders
pub async fn list_orders<S>(
    State(state): State<AppState<S>>,
    auth_session: AuthSession<S>,
    Query(query): Query<ListOrdersQuery>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: PurchaseOrderService + UserService + Clone,
{
    let user = auth_session.user.as_ref().ok_or(AppError::Unauthorized)?;

    let req = ListOrdersRequest {
        user_id: user.id(),
        role: query.role,
        status: query.status,
    };

    let orders = state
        .service
        .list_orders(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(orders)))
}

pub fn create_list_orders_docs(op: TransformOperation) -> TransformOperation {
    op.summary("List orders")
        .description("List purchase orders for the authenticated user.")
        .tag("Purchase Order")
        .response::<200, Json<Vec<PurchaseOrder>>>()
}

#[derive(Deserialize, JsonSchema)]
pub struct OrderIdPath {
    pub order_id: PurchaseOrderId,
}

/// GET /orders/{order_id}
pub async fn get_order<S>(
    State(state): State<AppState<S>>,
    auth_session: AuthSession<S>,
    Path(OrderIdPath { order_id }): Path<OrderIdPath>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: PurchaseOrderService + UserService + Clone,
{
    let user = auth_session.user.as_ref().ok_or(AppError::Unauthorized)?;

    let req = GetOrderRequest {
        user_id: user.id(),
        order_id,
    };

    let order = state
        .service
        .get_order(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(order)))
}

pub fn create_get_order_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get order")
        .description("Get a purchase order by ID.")
        .tag("Purchase Order")
        .response::<200, Json<PurchaseOrder>>()
}

/// POST /orders/{order_id}/items
pub async fn add_order_item<S>(
    State(state): State<AppState<S>>,
    auth_session: AuthSession<S>,
    Path(OrderIdPath { order_id }): Path<OrderIdPath>,
    Json(body): Json<AddOrderItemBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: PurchaseOrderService + UserService + Clone,
{
    let user = auth_session.user.as_ref().ok_or(AppError::Unauthorized)?;

    let req = AddOrderItemRequest {
        user_id: user.id(),
        order_id,
        variant_id: body.variant_id,
        owner_id: body.owner_id,
        quantity: body.quantity,
        unit_price: body.unit_price,
    };

    let item_id = state
        .service
        .add_order_item(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(item_id)))
}

pub fn create_add_order_item_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Add order item")
        .description("Add an item to a purchase order.")
        .tag("Purchase Order")
        .response::<201, Json<PurchaseOrderItemId>>()
}

#[derive(Deserialize, JsonSchema)]
pub struct OrderIdPathItemIdPath {
    pub order_id: PurchaseOrderId,
    pub item_id: PurchaseOrderItemId,
}

/// POST /orders/{order_id}/items/{item_id}/mystery-box
pub async fn submit_mystery_box_results<S>(
    State(state): State<AppState<S>>,
    auth_session: AuthSession<S>,
    Path(OrderIdPathItemIdPath { order_id, item_id }): Path<OrderIdPathItemIdPath>,
    Json(body): Json<SubmitMysteryBoxResultsBody>,
) -> Result<impl IntoApiResponse, AppError>
where
    S: PurchaseOrderService + UserService + Clone,
{
    let user = auth_session.user.as_ref().ok_or(AppError::Unauthorized)?;

    let req = SubmitMysteryBoxResultsRequest {
        user_id: user.id(),
        order_id,
        order_item_id: item_id,
        owner_id: body.owner_id,
        received_variants: body.received_variants,
    };

    state
        .service
        .submit_mystery_box_results(req)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(StatusCode::OK)
}

pub fn create_submit_mystery_box_results_docs(op: TransformOperation) -> TransformOperation {
    op.summary("Submit mystery box results")
        .description("Submit results for a mystery box item.")
        .tag("Purchase Order")
        .response::<200, ()>()
}
