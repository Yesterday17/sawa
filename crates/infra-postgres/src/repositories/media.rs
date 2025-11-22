use crate::{entities::media, error::DatabaseError, traits::TryIntoDomainModelSimple};
use sawa_core::{
    errors::RepositoryError,
    models::misc::{Media, MediaId},
    repositories::MediaRepository,
};
use sea_orm::{QueryFilter, prelude::*, sea_query::OnConflict};

pub struct PostgresMediaRepository {
    db: DatabaseConnection,
}

impl PostgresMediaRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl MediaRepository for PostgresMediaRepository {
    async fn find_by_id(&self, id: &MediaId) -> Result<Option<Media>, RepositoryError> {
        let entity = media::Entity::find_by_id(Uuid::from(id.0))
            .one(&self.db)
            .await
            .map_err(DatabaseError)?;

        entity
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .transpose()
    }

    async fn find_by_ids(&self, ids: &[MediaId]) -> Result<Vec<Media>, RepositoryError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let uuids: Vec<Uuid> = ids.iter().map(|id| Uuid::from(id.0)).collect();

        let entities = media::Entity::find()
            .filter(media::Column::Id.is_in(uuids))
            .all(&self.db)
            .await
            .map_err(DatabaseError)?;

        entities
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect()
    }

    async fn save(&self, media: &Media) -> Result<(), RepositoryError> {
        let media: media::ActiveModel = media.into();

        media::Entity::insert(media)
            .on_conflict(
                OnConflict::column(media::Column::Id)
                    .update_columns([media::Column::Url])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }

    async fn delete(&self, id: &MediaId) -> Result<(), RepositoryError> {
        media::Entity::delete_by_id(Uuid::from(id.0))
            .exec(&self.db)
            .await
            .map_err(DatabaseError)?;

        Ok(())
    }
}
