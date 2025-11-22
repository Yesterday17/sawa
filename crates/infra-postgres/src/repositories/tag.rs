use crate::{
    entities::tag::{Column, Entity},
    error::DatabaseError,
    traits::TryIntoDomainModelSimple,
};
use sawa_core::{
    errors::RepositoryError,
    models::misc::{Tag, TagId},
    repositories::TagRepository,
};
use sea_orm::{QueryFilter, prelude::*, sea_query::OnConflict};

pub struct PostgresTagRepository {
    db: DatabaseConnection,
}

impl PostgresTagRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl TagRepository for PostgresTagRepository {
    async fn find_by_id(&self, id: &TagId) -> Result<Option<Tag>, RepositoryError> {
        let entity = Entity::find_by_id(Uuid::from(id.0))
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_all(&self) -> Result<Vec<Tag>, RepositoryError> {
        let entities = Entity::find().all(&self.db).await.map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_name_prefix(&self, prefix: &str) -> Result<Vec<Tag>, RepositoryError> {
        let entities = Entity::find()
            .filter(Column::Name.ilike(&format!("{}%", prefix)))
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_by_parent(&self, parent_id: &TagId) -> Result<Vec<Tag>, RepositoryError> {
        let entities = Entity::find()
            .filter(Column::ParentTagId.eq(Uuid::from(parent_id.0)))
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn find_roots(&self) -> Result<Vec<Tag>, RepositoryError> {
        let entities = Entity::find()
            .filter(Column::ParentTagId.is_null())
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, tag: &Tag) -> Result<(), RepositoryError> {
        let active_model: crate::entities::tag::ActiveModel = tag.into();

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([Column::Name, Column::Description, Column::ParentTagId])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }

    async fn delete(&self, id: &TagId) -> Result<(), RepositoryError> {
        Entity::delete_by_id(Uuid::from(id.0))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}
