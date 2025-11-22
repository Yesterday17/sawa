use crate::{error::DatabaseError, traits::TryIntoDomainModelSimple};
use sawa_core::{errors::RepositoryError, models::misc::Tag};
use sea_orm::{ActiveValue, TryIntoModel, entity::prelude::*};

/// Tag entity
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    /// The unique identifier for the tag.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The name of the tag.
    #[sea_orm(unique)]
    pub name: String,

    /// Optional description of the tag.
    pub description: String,

    /// Parent tag for hierarchical organization (optional).
    pub parent_tag_id: Option<Uuid>,
    #[sea_orm(belongs_to, from = "parent_tag_id", to = "id", skip_fk)]
    pub parent_tag: HasOne<super::tag::Entity>,

    /// Product variants associated with this tag.
    #[sea_orm(has_many, via = "product_variant_tag", skip_fk)]
    pub product_variants: HasMany<super::product_variant::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryIntoDomainModelSimple<Tag> for Model {
    fn try_into_domain_model_simple(self) -> Result<Tag, RepositoryError> {
        Ok(Tag {
            id: self.id.try_into()?,
            name: self.name.try_into()?,
            description: self.description.clone(),
            parent_tag_id: match self.parent_tag_id {
                Some(id) => Some(id.try_into()?),
                None => None,
            },
        })
    }
}

impl TryIntoDomainModelSimple<Tag> for ModelEx {
    fn try_into_domain_model_simple(self) -> Result<Tag, RepositoryError> {
        self.try_into_model()
            .map_err(DatabaseError)?
            .try_into_domain_model_simple()
    }
}

impl From<&Tag> for crate::entities::tag::ActiveModel {
    fn from(tag: &Tag) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::from(tag.id.0)),
            name: ActiveValue::Set(tag.name.as_str().to_string()),
            description: ActiveValue::Set(tag.description.clone()),
            parent_tag_id: ActiveValue::Set(tag.parent_tag_id.map(Into::into)),
        }
    }
}
