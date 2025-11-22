use std::str::FromStr;

use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{errors::RepositoryResult, models::misc::Media};
use sea_orm::{ActiveValue, entity::prelude::*};
use url::Url;

///
/// Media entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The publicly accessible URL of the media.
    pub url: String,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<&Media> for crate::entities::media::ActiveModel {
    fn from(media: &Media) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::from(media.id.0)),
            url: ActiveValue::Set(media.url.to_string()),
        }
    }
}

impl TryIntoDomainModelSimple<Media> for Model {
    fn try_into_domain_model_simple(self) -> RepositoryResult<Media> {
        Ok(Media {
            id: self.id.try_into()?,
            url: Url::from_str(&self.url)?,
        })
    }
}
