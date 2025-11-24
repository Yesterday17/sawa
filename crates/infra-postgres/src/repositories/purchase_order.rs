use crate::{
    entities::purchase_order, error::DatabaseError, purchase_order_item, purchase_order_line_item,
    traits::TryIntoDomainModelSimple,
};
use sawa_core::{
    errors::RepositoryError,
    models::{
        purchase::{OrderRoleFilter, PurchaseOrder, PurchaseOrderId, PurchaseOrderStatus},
        user::UserId,
    },
    repositories::PurchaseOrderRepository,
};
use sea_orm::{ExprTrait, QueryFilter, TransactionTrait, prelude::*, sea_query::Query};
use std::collections::HashMap;

pub struct PostgresPurchaseOrderRepository {
    db: DatabaseConnection,
}

impl PostgresPurchaseOrderRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl PurchaseOrderRepository for PostgresPurchaseOrderRepository {
    async fn find_by_id(
        &self,
        id: &PurchaseOrderId,
        user_id: &UserId,
    ) -> Result<Option<PurchaseOrder>, RepositoryError> {
        pub use purchase_order::Column;

        let entity = purchase_order::Entity::load()
            .with((
                purchase_order_item::Entity,
                purchase_order_line_item::Entity,
            ))
            .filter(
                Column::Id.eq(Uuid::from(id.0)).and(
                    Column::CreatorId.eq(Uuid::from(user_id.0)) // Creator can access the order
                        .or(Column::ReceiverId.eq(Uuid::from(user_id.0))) // Receiver can access the order
                        .or(Column::Id.in_subquery( // Owners of any line item can access the order
                            Query::select()
                                .column(purchase_order_item::Column::PurchaseOrderId)
                                .from(purchase_order_item::Entity)
                                .and_where(
                                    purchase_order_item::Column::Id.in_subquery(
                                        Query::select()
                                            .column(purchase_order_line_item::Column::PurchaseOrderItemId)
                                            .from(purchase_order_line_item::Entity)
                                            .and_where(
                                                purchase_order_line_item::Column::OwnerId
                                                    .eq(Uuid::from(user_id.0)),
                                            ).to_owned()
                                    ),
                                )
                                .to_owned(),
                        )),
                ),
            )
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn load_by_ids(
        &self,
        ids: &[PurchaseOrderId],
        user_id: &UserId,
    ) -> Result<Vec<Option<PurchaseOrder>>, RepositoryError> {
        pub use purchase_order::Column;

        let uuid_ids: Vec<Uuid> = ids.iter().map(|id| Uuid::from(id.0)).collect();

        let entities = purchase_order::Entity::load()
            .with((
                purchase_order_item::Entity,
                purchase_order_line_item::Entity,
            ))
            .filter(purchase_order::Column::Id.is_in(uuid_ids).and(
                    Column::CreatorId.eq(Uuid::from(user_id.0)) // Creator can access the order
                        .or(Column::ReceiverId.eq(Uuid::from(user_id.0))) // Receiver can access the order
                        .or(Column::Id.in_subquery( // Owners of any line item can access the order
                            Query::select()
                                .column(purchase_order_item::Column::PurchaseOrderId)
                                .from(purchase_order_item::Entity)
                                .and_where(
                                    purchase_order_item::Column::Id.in_subquery(
                                        Query::select()
                                            .column(purchase_order_line_item::Column::PurchaseOrderItemId)
                                            .from(purchase_order_line_item::Entity)
                                            .and_where(
                                                purchase_order_line_item::Column::OwnerId
                                                    .eq(Uuid::from(user_id.0)),
                                            ).to_owned()
                                    ),
                                )
                                .to_owned(),
                        )),
                ))
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        let mut entity_map: HashMap<Uuid, PurchaseOrder> = entities
            .into_iter()
            .filter_map(|e| {
                e.try_into_domain_model_simple()
                    .ok()
                    .map(|model| (Uuid::from(model.id.0), model))
            })
            .collect();

        let result = ids
            .into_iter()
            .map(|id| entity_map.remove(&Uuid::from(id.0)))
            .collect();

        Ok(result)
    }

    async fn find_by_user(
        &self,
        user_id: &UserId,
        role: OrderRoleFilter,
        status: Option<PurchaseOrderStatus>,
    ) -> Result<Vec<PurchaseOrder>, RepositoryError> {
        let mut query = purchase_order::Entity::load();

        match role {
            OrderRoleFilter::Creator => {
                query = query.filter(purchase_order::Column::CreatorId.eq(Uuid::from(user_id.0)));
            }
            OrderRoleFilter::Receiver => {
                query = query.filter(purchase_order::Column::ReceiverId.eq(Uuid::from(user_id.0)));
            }
            OrderRoleFilter::Participant => {
                query = query.filter(
                    purchase_order::Column::CreatorId
                        .eq(Uuid::from(user_id.0))
                        .or(purchase_order::Column::ReceiverId.eq(Uuid::from(user_id.0)))
                        .or(purchase_order::Column::Id.in_subquery(
                            Query::select()
                                .column(purchase_order_item::Column::PurchaseOrderId)
                                .from(purchase_order_item::Entity)
                                .and_where(
                                    purchase_order_item::Column::Id.in_subquery(
                                        Query::select()
                                            .column(purchase_order_line_item::Column::PurchaseOrderItemId)
                                            .from(purchase_order_line_item::Entity)
                                            .and_where(
                                                purchase_order_line_item::Column::OwnerId
                                                    .eq(Uuid::from(user_id.0)),
                                            )
                                            .to_owned(),
                                    ),
                                )
                                .to_owned(),
                        )),
                );
            }
        }

        if let Some(status) = status {
            let db_status = match status {
                PurchaseOrderStatus::Incomplete => "incomplete",
                PurchaseOrderStatus::Fulfilled => "fulfilled",
                PurchaseOrderStatus::Cancelled => "cancelled",
            };
            query = query.filter(purchase_order::Column::Status.eq(db_status));
        }

        let entities = query
            .with(purchase_order_item::Entity)
            .with((
                purchase_order_item::Entity,
                purchase_order_line_item::Entity,
            ))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Internal(e.to_string()))?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, order: &PurchaseOrder) -> Result<(), RepositoryError> {
        let order_id = Uuid::from(order.id.0);
        let order_active_model: purchase_order::ActiveModel = order.into();

        // Prepare item data outside the closure
        let items_data: Vec<(
            purchase_order_item::ActiveModel,
            Vec<purchase_order_line_item::ActiveModel>,
        )> = order
            .items
            .iter()
            .map(|item| {
                let item_model: purchase_order_item::ActiveModel = (item, order_id).into();
                let line_item_models: Vec<purchase_order_line_item::ActiveModel> = item
                    .line_items
                    .iter()
                    .map(|line_item| line_item.into())
                    .collect();
                (item_model, line_item_models)
            })
            .collect();

        self.db
            .transaction(|db| {
                Box::pin(async move {
                    // Save or update the order
                    purchase_order::Entity::insert(order_active_model)
                        .on_conflict(
                            sea_orm::sea_query::OnConflict::column(purchase_order::Column::Id)
                                .update_columns([
                                    purchase_order::Column::CreatorId,
                                    purchase_order::Column::ReceiverId,
                                    purchase_order::Column::ShippingAddress,
                                    purchase_order::Column::TotalPriceCurrency,
                                    purchase_order::Column::TotalPriceAmount,
                                    purchase_order::Column::Status,
                                    purchase_order::Column::CompletedAt,
                                    purchase_order::Column::CancelledAt,
                                ])
                                .to_owned(),
                        )
                        .exec(db)
                        .await?;

                    // Delete existing line items first (due to foreign key constraints)
                    purchase_order_line_item::Entity::delete_many()
                        .filter(
                            purchase_order_line_item::Column::PurchaseOrderItemId.in_subquery(
                                sea_orm::sea_query::Query::select()
                                    .column(purchase_order_item::Column::Id)
                                    .from(purchase_order_item::Entity)
                                    .and_where(
                                        purchase_order_item::Column::PurchaseOrderId.eq(order_id),
                                    )
                                    .to_owned(),
                            ),
                        )
                        .exec(db)
                        .await?;

                    // Delete existing items
                    purchase_order_item::Entity::delete_many()
                        .filter(purchase_order_item::Column::PurchaseOrderId.eq(order_id))
                        .exec(db)
                        .await?;

                    // Save all items and their line items
                    for (item_model, line_item_models) in items_data {
                        purchase_order_item::Entity::insert(item_model)
                            .exec(db)
                            .await?;

                        // Save line items for this item
                        if !line_item_models.is_empty() {
                            purchase_order_line_item::Entity::insert_many(line_item_models)
                                .exec(db)
                                .await?;
                        }
                    }

                    Ok(())
                })
            })
            .await
            .map_err(DatabaseError::from)?;

        Ok(())
    }

    async fn delete(&self, id: &PurchaseOrderId) -> Result<(), RepositoryError> {
        let id = Uuid::from(id.0);
        self.db
            .transaction(|db| {
                Box::pin(async move {
                    // Delete line items based on items.order_id = id
                    purchase_order_line_item::Entity::delete_many()
                        .filter(
                            purchase_order_line_item::Column::PurchaseOrderItemId.in_subquery(
                                Query::select()
                                    .column(purchase_order_item::Column::Id)
                                    .from(purchase_order_item::Entity)
                                    .and_where(purchase_order_item::Column::PurchaseOrderId.eq(id))
                                    .to_owned(),
                            ),
                        )
                        .exec(db)
                        .await?;
                    // Delete items based on order_id = id
                    purchase_order_item::Entity::delete_many()
                        .filter(purchase_order_item::Column::PurchaseOrderId.eq(id))
                        .exec(db)
                        .await?;
                    // Finally delete the order
                    purchase_order::Entity::delete_by_id(id).exec(db).await?;

                    Ok(())
                })
            })
            .await
            .map_err(DatabaseError::from)?;

        Ok(())
    }
}
