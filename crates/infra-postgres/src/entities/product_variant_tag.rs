use sea_orm::entity::prelude::*;

///
/// ProductVariantTag entity (many-to-many junction table)
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "product_variant_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub product_variant_id: Uuid,

    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: Uuid,

    #[sea_orm(belongs_to, from = "product_variant_id", to = "id")]
    pub product_variant: HasOne<super::product_variant::Entity>,
    #[sea_orm(belongs_to, from = "tag_id", to = "id")]
    pub tag: HasOne<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
