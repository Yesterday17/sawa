use sea_orm::entity::prelude::*;

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
    #[sea_orm(belongs_to, from = "parent_tag_id", to = "id")]
    pub parent_tag: HasOne<super::tag::Entity>,

    /// Product variants associated with this tag.
    #[sea_orm(has_many, via = "product_variant_tag")]
    pub product_variants: HasMany<super::product_variant::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
