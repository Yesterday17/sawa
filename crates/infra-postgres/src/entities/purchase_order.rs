use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{
    errors::RepositoryError,
    models::{
        misc::{Currency, Price},
        purchase::{PurchaseOrder, PurchaseOrderStatus},
    },
};
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

///
/// PurchaseOrder entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "purchase_orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The user who created the order.
    pub creator_id: Uuid,
    #[sea_orm(
        belongs_to,
        relation_enum = "Creator",
        from = "creator_id",
        to = "id",
        skip_fk
    )]
    pub creator: HasOne<super::user::Entity>,

    /// The user who will receive the items.
    pub receiver_id: Uuid,
    #[sea_orm(
        belongs_to,
        relation_enum = "Receiver",
        from = "receiver_id",
        to = "id",
        skip_fk
    )]
    pub receiver: HasOne<super::user::Entity>,

    /// The items being purchased
    #[sea_orm(has_many, skip_fk)]
    pub items: HasMany<super::purchase_order_item::Entity>,

    /// Shipping/delivery address (if physical goods)
    #[sea_orm(column_type = "JsonBinary")]
    pub shipping_address: Option<DBAddress>,

    /// Total amount paid
    pub total_price_currency: String,
    pub total_price_amount: i64,

    /// Current status of the order
    pub status: DBPurchaseOrderStatus,

    /// The timestamp when the order was created.
    pub created_at: DateTimeUtc,

    /// The timestamp when the order was completed.
    pub completed_at: Option<DateTimeUtc>,

    /// The timestamp when the order was cancelled.
    pub cancelled_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DBPurchaseOrderStatus {
    Incomplete,
    Fulfilled,
    Cancelled,
}

impl From<PurchaseOrderStatus> for DBPurchaseOrderStatus {
    fn from(status: PurchaseOrderStatus) -> Self {
        match status {
            PurchaseOrderStatus::Incomplete => DBPurchaseOrderStatus::Incomplete,
            PurchaseOrderStatus::Fulfilled => DBPurchaseOrderStatus::Fulfilled,
            PurchaseOrderStatus::Cancelled => DBPurchaseOrderStatus::Cancelled,
        }
    }
}

impl From<DBPurchaseOrderStatus> for PurchaseOrderStatus {
    fn from(db_status: DBPurchaseOrderStatus) -> Self {
        match db_status {
            DBPurchaseOrderStatus::Incomplete => PurchaseOrderStatus::Incomplete,
            DBPurchaseOrderStatus::Fulfilled => PurchaseOrderStatus::Fulfilled,
            DBPurchaseOrderStatus::Cancelled => PurchaseOrderStatus::Cancelled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct DBAddress(pub sawa_core::models::misc::Address);

impl DBAddress {
    pub fn new(address: sawa_core::models::misc::Address) -> Self {
        Self(address)
    }

    pub fn into_inner(self) -> sawa_core::models::misc::Address {
        self.0
    }

    pub fn as_ref(&self) -> &sawa_core::models::misc::Address {
        &self.0
    }
}

impl TryIntoDomainModelSimple<PurchaseOrder> for ModelEx {
    fn try_into_domain_model_simple(self) -> Result<PurchaseOrder, RepositoryError> {
        let items = self
            .items
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PurchaseOrder {
            id: self.id.try_into()?,
            creator_id: self.creator_id.try_into()?,
            receiver_id: self.receiver_id.try_into()?,
            items,
            shipping_address: self.shipping_address.map(|addr| addr.into_inner()),
            total_price: Price {
                currency: Currency::from_str(&self.total_price_currency)?,
                amount: self.total_price_amount as u32,
            },
            status: self.status.into(),
            created_at: self.created_at,
            completed_at: self.completed_at,
            cancelled_at: self.cancelled_at,
        })
    }
}

impl From<&PurchaseOrder> for ActiveModel {
    fn from(order: &PurchaseOrder) -> Self {
        Self {
            id: Set(Uuid::from(order.id.0)),
            creator_id: Set(Uuid::from(order.creator_id.0)),
            receiver_id: Set(Uuid::from(order.receiver_id.0)),
            shipping_address: Set(order
                .shipping_address
                .as_ref()
                .map(|addr| DBAddress::new(addr.clone()))),
            total_price_currency: Set(order.total_price.currency.code().to_string()),
            total_price_amount: Set(order.total_price.amount as i64),
            status: Set(order.status.into()),
            created_at: Set(order.created_at),
            completed_at: Set(order.completed_at),
            cancelled_at: Set(order.cancelled_at),
        }
    }
}
