use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{errors::RepositoryError, models::product::Product};
use sea_orm::{ActiveValue::Set, entity::prelude::*};

///
/// Product entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Name of the product.
    pub name: String,

    /// Description of the product.
    pub description: String,

    /// Media IDs associated with the product.
    pub medias: Vec<Uuid>,

    /// A product can have many variants.
    #[sea_orm(has_many, skip_fk)]
    pub variants: HasMany<super::product_variant::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryIntoDomainModelSimple<Product> for Model {
    fn try_into_domain_model_simple(self) -> Result<Product, RepositoryError> {
        Ok(Product {
            id: self.id.try_into()?,
            name: self.name.try_into()?,
            description: self.description.clone(),
            medias: self
                .medias
                .into_iter()
                .map(|id| id.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<&Product> for crate::entities::product::ActiveModel {
    fn from(product: &Product) -> Self {
        Self {
            id: Set(product.id.into()),
            name: Set(product.name.clone().into()),
            description: Set(product.description.clone()),
            medias: Set(product.medias.iter().map(|id| Uuid::from(id.0)).collect()),
        }
    }
}
