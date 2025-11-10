use sawa_core::models::product::MysteryBoxConfig;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

///
/// ProductVariant entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_variants")]
pub struct Model {
    /// The unique identifier for the product variant.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The ID of the product to which this variant belongs.
    pub product_id: Uuid,
    #[sea_orm(belongs_to, from = "product_id", to = "id")]
    pub product: HasOne<super::product::Entity>,

    /// The name of the product variant.
    pub name: String,

    /// The description of the product variant.
    pub description: String,

    /// The media items associated with the product variant.
    pub medias: Vec<Uuid>,

    /// Tags associated with this variant (characters, series, themes, etc.).
    #[sea_orm(has_many, via = "product_variant_tag")]
    pub tags: HasMany<super::tag::Entity>,

    /// The price of the product variant.
    pub price_currency: Option<String>,
    pub price_amount: Option<u64>,

    /// The mystery box configuration of the product variant.
    #[sea_orm(column_type = "JsonBinary")]
    pub mystery_box: Option<DBMysteryBoxConfig>,

    /// The sort order of the product variant among other variants of the same product.
    /// Variants with lower order values should be displayed before those with higher values.
    ///
    /// Variants with the same order value would be displayed in the order of variant_id.
    pub sort_order: i32,

    /// A product variant can have many instances.
    #[sea_orm(has_many)]
    pub instances: HasMany<super::product_instance::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct DBMysteryBoxConfig(MysteryBoxConfig);
