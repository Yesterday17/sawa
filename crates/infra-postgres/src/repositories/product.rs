use crate::{
    entities::{product, product_variant, product_variant_tag},
    error::DatabaseError,
    tag,
    traits::TryIntoDomainModelSimple,
};
use sawa_core::{
    errors::RepositoryError,
    models::{
        misc::TagId,
        product::{Product, ProductId, ProductVariant, ProductVariantId},
    },
    repositories::{ProductRepository, ProductVariantRepository},
};
use sea_orm::{
    FromQueryResult, QueryFilter, QueryOrder, TransactionTrait, prelude::*, raw_sql,
    sea_query::OnConflict,
};

pub struct PostgresProductRepository {
    db: DatabaseConnection,
}

impl PostgresProductRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ProductRepository for PostgresProductRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError> {
        let entity = product::Entity::find_by_id(Uuid::from(id.0))
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let entities = product::Entity::find()
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, product: &Product) -> Result<(), RepositoryError> {
        let active_model: crate::entities::product::ActiveModel = product.into();

        product::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(product::Column::Id)
                    .update_columns([
                        product::Column::Name,
                        product::Column::Description,
                        product::Column::Medias,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }

    async fn delete(&self, id: &ProductId) -> Result<(), RepositoryError> {
        product::Entity::delete_by_id(Uuid::from(id.0))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}

pub struct PostgresProductVariantRepository {
    db: DatabaseConnection,
}

impl PostgresProductVariantRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ProductVariantRepository for PostgresProductVariantRepository {
    async fn find_by_id(
        &self,
        id: &ProductVariantId,
    ) -> Result<Option<ProductVariant>, RepositoryError> {
        let entity = product_variant::Entity::load()
            .filter(product_variant::Column::Id.eq(Uuid::from(id.0)))
            .with(tag::Entity)
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<ProductVariant>, RepositoryError> {
        let entities = product_variant::Entity::load()
            .filter(product_variant::Column::ProductId.eq(Uuid::from(product_id.0)))
            .with(tag::Entity)
            .order_by_asc(product_variant::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_tags_all(
        &self,
        tag_ids: &[TagId],
    ) -> Result<Vec<ProductVariant>, RepositoryError> {
        if tag_ids.is_empty() {
            return Ok(vec![]);
        }

        let tag_uuids: Vec<Uuid> = tag_ids.iter().map(|id| Uuid::from(id.0)).collect();
        let tag_uuids_len = tag_uuids.len() as u32;

        // Find variants with all the tags
        #[derive(FromQueryResult)]
        struct Data {
            id: Uuid,
        }
        let sql = raw_sql!(
            Postgres,
            r#"
            SELECT
              "product_variants"."id"
            FROM
              "product_variants"
            WHERE "id" IN (
              SELECT
                "product_variant_id"
              FROM
                "product_variant_tags"
              WHERE
                "tag_id" IN ({..tag_uuids})
              GROUP BY
                "product_variant_id"
              HAVING
                COUNT(*) = {tag_uuids_len}
            )"#
        );
        let variant_ids: Vec<Uuid> = Data::find_by_statement(sql)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?
            .into_iter()
            .map(|v| v.id)
            .collect();

        if variant_ids.is_empty() {
            return Ok(vec![]);
        }

        let entities = product_variant::Entity::load()
            .filter(product_variant::Column::Id.is_in(variant_ids))
            .with(tag::Entity)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_tags_any(
        &self,
        tag_ids: &[TagId],
    ) -> Result<Vec<ProductVariant>, RepositoryError> {
        if tag_ids.is_empty() {
            return Ok(vec![]);
        }

        let tag_uuids: Vec<Uuid> = tag_ids.iter().map(|id| Uuid::from(id.0)).collect();
        let variant_ids: Vec<Uuid> = product_variant_tag::Entity::find()
            .filter(product_variant_tag::Column::TagId.is_in(tag_uuids))
            .all(&self.db)
            .await
            .map_err(DatabaseError)?
            .into_iter()
            .map(|v| v.product_variant_id)
            .collect();

        if variant_ids.is_empty() {
            return Ok(vec![]);
        }

        let entities = product_variant::Entity::load()
            .filter(product_variant::Column::Id.is_in(variant_ids))
            .with(tag::Entity)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_all(&self) -> Result<Vec<ProductVariant>, RepositoryError> {
        let entities = product_variant::Entity::load()
            .with(tag::Entity)
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, variant: &ProductVariant) -> Result<(), RepositoryError> {
        let variant_id = Uuid::from(variant.id.0);
        let variant_active_model: product_variant::ActiveModel = variant
            .try_into()
            .map_err(|e| RepositoryError::Internal(format!("Failed to convert variant: {}", e)))?;

        // Prepare tag associations
        let tag_models: Vec<_> = variant
            .tags
            .iter()
            .map(|tag_id| product_variant_tag::ActiveModel {
                product_variant_id: sea_orm::ActiveValue::Set(variant_id),
                tag_id: sea_orm::ActiveValue::Set(Uuid::from(tag_id.0)),
            })
            .collect();

        self.db
            .transaction(|db| {
                Box::pin(async move {
                    // Save or update the variant
                    product_variant::Entity::insert(variant_active_model)
                        .on_conflict(
                            OnConflict::column(product_variant::Column::Id)
                                .update_columns([
                                    product_variant::Column::ProductId,
                                    product_variant::Column::Name,
                                    product_variant::Column::Description,
                                    product_variant::Column::Medias,
                                    product_variant::Column::PriceCurrency,
                                    product_variant::Column::PriceAmount,
                                    product_variant::Column::MysteryBox,
                                    product_variant::Column::SortOrder,
                                ])
                                .to_owned(),
                        )
                        .exec(db)
                        .await?;

                    // Delete existing tag associations
                    product_variant_tag::Entity::delete_many()
                        .filter(product_variant_tag::Column::ProductVariantId.eq(variant_id))
                        .exec(db)
                        .await?;

                    // Insert new tag associations
                    if !tag_models.is_empty() {
                        product_variant_tag::Entity::insert_many(tag_models)
                            .exec(db)
                            .await?;
                    }

                    Ok(())
                })
            })
            .await
            .map_err(DatabaseError::from)?;

        Ok(())
    }

    async fn delete(&self, id: &ProductVariantId) -> Result<(), RepositoryError> {
        product_variant::Entity::delete_by_id(Uuid::from(id.0))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;
        product_variant_tag::Entity::delete_many()
            .filter(product_variant_tag::Column::ProductVariantId.eq(Uuid::from(id.0)))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}
