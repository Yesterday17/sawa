use sawa_core::{
    errors::RepositoryError,
    models::{
        product::{ProductInstance, ProductInstanceId, ProductInstanceStatus, ProductVariantId},
        purchase::PurchaseOrderLineItemId,
        user::UserId,
    },
    repositories::ProductInstanceRepository,
};
use sea_orm::{QueryFilter, prelude::*, sea_query::OnConflict};

use crate::{
    error::DatabaseError, product_instance, product_instance_status_history,
    product_instance_transfer_history, traits::TryIntoDomainModelSimple,
};

pub struct PostgresProductInstanceRepository {
    db: DatabaseConnection,
}

impl PostgresProductInstanceRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ProductInstanceRepository for PostgresProductInstanceRepository {
    async fn find_by_id(
        &self,
        id: &ProductInstanceId,
    ) -> Result<Option<ProductInstance>, RepositoryError> {
        let entity = product_instance::Entity::load()
            .filter(product_instance::Column::Id.eq(Uuid::from(id.0)))
            .with(product_instance_transfer_history::Entity)
            .with(product_instance_status_history::Entity)
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_by_line_item_id(
        &self,
        line_item_id: &PurchaseOrderLineItemId,
    ) -> Result<Option<ProductInstance>, RepositoryError> {
        let entity = product_instance::Entity::load()
            .filter(product_instance::Column::SourceOrderLineItemId.eq(Uuid::from(line_item_id.0)))
            .with(product_instance_transfer_history::Entity)
            .with(product_instance_status_history::Entity)
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_by_owner(
        &self,
        owner_id: &UserId,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let entities = product_instance::Entity::load()
            .filter(product_instance::Column::OwnerId.eq(Uuid::from(owner_id.0)))
            .with(product_instance_transfer_history::Entity)
            .with(product_instance_status_history::Entity)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_owner_and_variant(
        &self,
        owner_id: &UserId,
        variant_id: &ProductVariantId,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let entities = product_instance::Entity::load()
            .filter(product_instance::Column::OwnerId.eq(Uuid::from(owner_id.0)))
            .filter(product_instance::Column::VariantId.eq(Uuid::from(variant_id.0)))
            .with(product_instance_transfer_history::Entity)
            .with(product_instance_status_history::Entity)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_owner_and_status(
        &self,
        owner_id: &UserId,
        status: ProductInstanceStatus,
    ) -> Result<Vec<ProductInstance>, RepositoryError> {
        let db_status: crate::entities::product_instance::DBProductInstanceStatus = status.into();

        let entities = product_instance::Entity::load()
            .filter(product_instance::Column::OwnerId.eq(Uuid::from(owner_id.0)))
            .filter(product_instance::Column::Status.eq(db_status))
            .with(product_instance_transfer_history::Entity)
            .with(product_instance_status_history::Entity)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, instance: &ProductInstance) -> Result<(), RepositoryError> {
        // TODO: Save with transaction
        let active_model: crate::entities::product_instance::ActiveModel = instance.into();
        product_instance::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(product_instance::Column::Id)
                    .update_columns([
                        product_instance::Column::OwnerId,
                        product_instance::Column::VariantId,
                        product_instance::Column::Status,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        // Remove existing transfer history and insert new ones
        product_instance_transfer_history::Entity::delete_many()
            .filter(
                product_instance_transfer_history::Column::ProductInstanceId
                    .eq(Uuid::from(instance.id.0)),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;
        for transfer in instance.transfer_history.iter() {
            let active_model: crate::entities::product_instance_transfer_history::ActiveModel =
                (transfer, instance.id).into();
            product_instance_transfer_history::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(DatabaseError)?;
        }

        // Remove existing status history and insert new ones
        product_instance_status_history::Entity::delete_many()
            .filter(
                product_instance_status_history::Column::ProductInstanceId
                    .eq(Uuid::from(instance.id.0)),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;
        for status in instance.status_history.iter() {
            let active_model: crate::entities::product_instance_status_history::ActiveModel =
                (status, instance.id).into();
            product_instance_status_history::Entity::insert(active_model)
                .exec(&self.db)
                .await
                .map_err(DatabaseError)?;
        }

        Ok(())
    }

    async fn save_batch(&self, instances: &[ProductInstance]) -> Result<(), RepositoryError> {
        todo!("Implement save_batch")
    }

    async fn delete(&self, id: &ProductInstanceId) -> Result<(), RepositoryError> {
        product_instance::Entity::delete_by_id(Uuid::from(id.0))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}
