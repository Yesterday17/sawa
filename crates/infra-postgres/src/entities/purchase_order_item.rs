use sawa_core::models::purchase::PurchaseOrderItemStatus;
use sea_orm::entity::prelude::*;

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
    #[sea_orm(belongs_to, from = "purchase_order_id", to = "id")]
    pub purchase_order: HasOne<super::purchase_order::Entity>,

    /// The variant of the product to purchase
    pub purchased_variant_id: Uuid,
    #[sea_orm(belongs_to, from = "purchased_variant_id", to = "id")]
    pub purchased_variant: HasOne<super::product_variant::Entity>,

    /// All items in this order item
    #[sea_orm(has_many)]
    pub line_items: HasMany<super::purchase_order_line_item::Entity>,

    /// Item status
    pub status: DBPurchaseOrderItemStatus,

    /// How many units to purchase
    pub quantity: u32,

    /// Price at time of order (snapshot, immutable)
    pub unit_price_currency: Option<String>,
    pub unit_price_amount: Option<u64>,
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
