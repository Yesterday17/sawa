use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{
    errors::RepositoryError,
    models::{
        misc::{Currency, Price},
        product::{MysteryBoxConfig, ProductVariant},
    },
};
use sea_orm::{ActiveValue::Set, entity::prelude::*};
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
pub struct DBMysteryBoxConfig(pub MysteryBoxConfig);

impl TryIntoDomainModelSimple<ProductVariant> for ModelEx {
    fn try_into_domain_model_simple(self) -> Result<ProductVariant, RepositoryError> {
        let tags = self
            .tags
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect::<Result<Vec<_>, RepositoryError>>()?
            .into_iter()
            .map(|tag| tag.id)
            .collect();

        let price: Option<Price> = match (&self.price_currency, self.price_amount) {
            (Some(currency_str), Some(amount)) => {
                let currency = currency_str
                    .parse::<Currency>()
                    .map_err(|e| RepositoryError::Internal(format!("Invalid currency: {}", e)))?;
                Some(Price { currency, amount })
            }
            _ => None,
        };

        Ok(ProductVariant {
            id: self.id.try_into()?,
            product_id: self.product_id.try_into()?,
            name: self.name.try_into()?,
            description: self.description.clone(),
            medias: self
                .medias
                .into_iter()
                .map(|id| id.try_into())
                .collect::<Result<Vec<_>, _>>()?,
            tags,
            price,
            mystery_box: self.mystery_box.map(|m| m.0),
            sort_order: self.sort_order,
        })
    }
}

impl TryFrom<&ProductVariant> for crate::entities::product_variant::ActiveModel {
    type Error = RepositoryError;

    fn try_from(variant: &ProductVariant) -> Result<Self, Self::Error> {
        use crate::entities::product_variant::DBMysteryBoxConfig;

        let mystery_box_db = variant
            .mystery_box
            .as_ref()
            .map(|config| DBMysteryBoxConfig(config.clone()));

        let (price_currency, price_amount) = match &variant.price {
            Some(price) => (Some(price.currency.to_string()), Some(price.amount)),
            None => (None, None),
        };

        Ok(Self {
            id: Set(Uuid::from(variant.id.0)),
            product_id: Set(Uuid::from(variant.product_id.0)),
            name: Set(variant.name.as_str().to_string()),
            description: Set(variant.description.clone()),
            medias: Set(variant.medias.iter().map(|id| Uuid::from(id.0)).collect()),
            price_currency: Set(price_currency),
            price_amount: Set(price_amount),
            mystery_box: Set(mystery_box_db),
            sort_order: Set(variant.sort_order),
        })
    }
}
