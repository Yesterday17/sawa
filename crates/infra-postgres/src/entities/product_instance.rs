use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{
    errors::RepositoryResult,
    models::product::{ProductInstance, ProductInstanceStatus},
};
use sea_orm::{ActiveValue, entity::prelude::*};

///
/// ProductInstance entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_instances")]
pub struct Model {
    /// The unique identifier for this specific product instance.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The ID of the product variant to which this instance belongs.
    pub variant_id: Uuid,
    #[sea_orm(belongs_to, from = "variant_id", to = "id")]
    pub variant: HasOne<super::product_variant::Entity>,

    /// The owner of this product instance.
    pub owner_id: Uuid,
    #[sea_orm(belongs_to, derive_enum = "Owner", from = "owner_id", to = "id")]
    pub owner: HasOne<super::user::Entity>,

    pub holder_id: Uuid,
    #[sea_orm(belongs_to, derive_enum = "Holder", from = "holder_id", to = "id")]
    pub holder: HasOne<super::user::Entity>,

    /// Status of this instance
    pub status: DBProductInstanceStatus,

    /// The line item that created this instance
    #[sea_orm(unique)]
    pub source_order_line_item_id: Uuid,
    #[sea_orm(belongs_to, from = "source_order_line_item_id", to = "id")]
    pub source_order_line_item: HasOne<super::purchase_order_line_item::Entity>,

    /// When this instance was created (when order was fulfilled)
    pub created_at: DateTimeUtc,

    /// Transfer history (optional, for auditing)
    #[sea_orm(has_many)]
    pub transfer_history: HasMany<super::product_instance_transfer_history::Entity>,

    /// Status history
    #[sea_orm(has_many)]
    pub status_history: HasMany<super::product_instance_status_history::Entity>,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DBProductInstanceStatus {
    Active,
    Locked,
    Consumed,
    NotFound,
    Destroyed,
}

impl From<DBProductInstanceStatus> for ProductInstanceStatus {
    fn from(db_status: DBProductInstanceStatus) -> Self {
        match db_status {
            DBProductInstanceStatus::Active => ProductInstanceStatus::Active,
            DBProductInstanceStatus::Locked => ProductInstanceStatus::Locked,
            DBProductInstanceStatus::Consumed => ProductInstanceStatus::Consumed,
            DBProductInstanceStatus::NotFound => ProductInstanceStatus::NotFound,
            DBProductInstanceStatus::Destroyed => ProductInstanceStatus::Destroyed,
        }
    }
}

impl From<ProductInstanceStatus> for DBProductInstanceStatus {
    fn from(status: ProductInstanceStatus) -> Self {
        match status {
            ProductInstanceStatus::Active => DBProductInstanceStatus::Active,
            ProductInstanceStatus::Locked => DBProductInstanceStatus::Locked,
            ProductInstanceStatus::Consumed => DBProductInstanceStatus::Consumed,
            ProductInstanceStatus::NotFound => DBProductInstanceStatus::NotFound,
            ProductInstanceStatus::Destroyed => DBProductInstanceStatus::Destroyed,
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl TryIntoDomainModelSimple<ProductInstance> for ModelEx {
    fn try_into_domain_model_simple(self) -> RepositoryResult<ProductInstance> {
        let transfer_history = self
            .transfer_history
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect::<Result<Vec<_>, _>>()?;
        let status_history = self
            .status_history
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ProductInstance {
            id: self.id.try_into()?,
            variant_id: self.variant_id.try_into()?,
            owner_id: self.owner_id.try_into()?,
            holder_id: self.holder_id.try_into()?,
            status: self.status.into(),
            source_order_line_item_id: self.source_order_line_item_id.try_into()?,
            created_at: self.created_at,
            transfer_history,
            status_history,
        })
    }
}

impl From<&ProductInstance> for crate::entities::product_instance::ActiveModel {
    fn from(instance: &ProductInstance) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::from(instance.id.0)),
            variant_id: ActiveValue::Set(Uuid::from(instance.variant_id.0)),
            owner_id: ActiveValue::Set(Uuid::from(instance.owner_id.0)),
            holder_id: ActiveValue::Set(Uuid::from(instance.holder_id.0)),
            status: ActiveValue::Set(instance.status.into()),
            source_order_line_item_id: ActiveValue::Set(Uuid::from(
                instance.source_order_line_item_id.0,
            )),
            created_at: ActiveValue::Set(instance.created_at),
        }
    }
}
