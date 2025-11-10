use sea_orm::entity::prelude::*;

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
    #[sea_orm(has_many)]
    pub variants: HasMany<super::product_variant::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
