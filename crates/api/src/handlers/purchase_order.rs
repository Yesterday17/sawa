use crate::{auth::AuthSession, error::AppError, state::AppState};
use aide::axum::IntoApiResponse;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use axum_login::AuthUser;
use sawa_core::{
    models::{
        misc::{Address, Price},
        product::ProductVariantId,
        purchase::{OrderRoleFilter, PurchaseOrderStatus},
        user::UserId,
    },
    services::{
        CreateOrderItemRequest, CreateOrderRequest, ListOrdersRequest, PurchaseOrderService,
        UserService,
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
