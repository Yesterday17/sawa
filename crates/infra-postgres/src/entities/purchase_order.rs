use sawa_core::models::purchase::PurchaseOrderStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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
    #[sea_orm(belongs_to, relation_enum = "Creator", from = "creator_id", to = "id")]
    pub creator: HasOne<super::user::Entity>,

    /// The user who will receive the items.
    pub receiver_id: Uuid,
    #[sea_orm(
        belongs_to,
        relation_enum = "Receiver",
        from = "receiver_id",
        to = "id"
    )]
    pub receiver: HasOne<super::user::Entity>,

    /// The items being purchased
    #[sea_orm(has_many)]
    pub items: HasMany<super::purchase_order_item::Entity>,

    /// Shipping/delivery address (if physical goods)
    #[sea_orm(column_type = "JsonBinary")]
    pub shipping_address: Option<DBAddress>,

    /// Total amount paid
    pub total_price_currency: String,
    pub total_price_amount: u64,

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
pub struct DBAddress(sawa_core::models::misc::Address);
