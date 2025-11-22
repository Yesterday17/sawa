use crate::traits::TryIntoDomainModelSimple;
use sawa_core::{
    errors::RepositoryError,
    models::{
        misc::{Currency, Price},
        purchase::{PurchaseOrderItem, PurchaseOrderItemStatus},
    },
};
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use std::str::FromStr;

///
/// PurchaseOrderItem entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "purchase_order_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The order this item belongs to
    pub purchase_order_id: Uuid,
    #[sea_orm(belongs_to, from = "purchase_order_id", to = "id", skip_fk)]
    pub purchase_order: HasOne<super::purchase_order::Entity>,

    /// The variant of the product to purchase
    pub purchased_variant_id: Uuid,
    #[sea_orm(belongs_to, from = "purchased_variant_id", to = "id", skip_fk)]
    pub purchased_variant: HasOne<super::product_variant::Entity>,

    /// All items in this order item
    #[sea_orm(has_many, skip_fk)]
    pub line_items: HasMany<super::purchase_order_line_item::Entity>,

    /// Item status
    pub status: DBPurchaseOrderItemStatus,

    /// How many units to purchase
    pub quantity: u32,

    /// Price at time of order (snapshot, immutable)
    pub unit_price_currency: Option<String>,
    pub unit_price_amount: Option<u32>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DBPurchaseOrderItemStatus {
    AwaitingInput,
    Pending,
    Fulfilled,
    Cancelled,
}

impl From<PurchaseOrderItemStatus> for DBPurchaseOrderItemStatus {
    fn from(status: PurchaseOrderItemStatus) -> Self {
        match status {
            PurchaseOrderItemStatus::AwaitingInput => DBPurchaseOrderItemStatus::AwaitingInput,
            PurchaseOrderItemStatus::Pending => DBPurchaseOrderItemStatus::Pending,
            PurchaseOrderItemStatus::Fulfilled => DBPurchaseOrderItemStatus::Fulfilled,
            PurchaseOrderItemStatus::Cancelled => DBPurchaseOrderItemStatus::Cancelled,
        }
    }
}

impl From<DBPurchaseOrderItemStatus> for PurchaseOrderItemStatus {
    fn from(db_status: DBPurchaseOrderItemStatus) -> Self {
        match db_status {
            DBPurchaseOrderItemStatus::AwaitingInput => PurchaseOrderItemStatus::AwaitingInput,
            DBPurchaseOrderItemStatus::Pending => PurchaseOrderItemStatus::Pending,
            DBPurchaseOrderItemStatus::Fulfilled => PurchaseOrderItemStatus::Fulfilled,
            DBPurchaseOrderItemStatus::Cancelled => PurchaseOrderItemStatus::Cancelled,
        }
    }
}

impl TryIntoDomainModelSimple<PurchaseOrderItem> for ModelEx {
    fn try_into_domain_model_simple(self) -> Result<PurchaseOrderItem, RepositoryError> {
        let line_items = self
            .line_items
            .into_iter()
            .map(TryIntoDomainModelSimple::try_into_domain_model_simple)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PurchaseOrderItem {
            id: self.id.try_into()?,
            purchased_variant_id: self.purchased_variant_id.try_into()?,
            line_items,
            status: self.status.into(),
            quantity: self.quantity.try_into()?,
            unit_price: match (self.unit_price_currency, self.unit_price_amount) {
                (Some(currency), Some(amount)) => Some(Price {
                    currency: Currency::from_str(&currency)?,
                    amount,
                }),
                _ => None,
            },
        })
    }
}

impl From<(&PurchaseOrderItem, Uuid)> for ActiveModel {
    fn from((item, purchase_order_id): (&PurchaseOrderItem, Uuid)) -> Self {
        Self {
            id: Set(Uuid::from(item.id.0)),
            purchase_order_id: Set(purchase_order_id),
            purchased_variant_id: Set(Uuid::from(item.purchased_variant_id.0)),
            status: Set(item.status.into()),
            quantity: Set(item.quantity.get()),
            unit_price_currency: Set(item
                .unit_price
                .as_ref()
                .map(|p| p.currency.code().to_string())),
            unit_price_amount: Set(item.unit_price.as_ref().map(|p| p.amount)),
        }
    }
}
