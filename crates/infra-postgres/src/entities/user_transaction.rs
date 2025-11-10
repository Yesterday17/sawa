use sawa_core::models::transfer::UserTransactionStatus;
use sea_orm::entity::prelude::*;

///
/// UserTransaction entity
///
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// User transferring items out
    pub from_user_id: Uuid,
    #[sea_orm(belongs_to, derive_enum = "FromUser", from = "from_user_id", to = "id")]
    pub from_user: HasOne<super::user::Entity>,

    /// User receiving items
    pub to_user_id: Uuid,
    #[sea_orm(belongs_to, derive_enum = "ToUser", from = "to_user_id", to = "id")]
    pub to_user: HasOne<super::user::Entity>,

    /// Items being transferred
    #[sea_orm(has_many)]
    pub items: HasMany<super::user_transaction_item::Entity>,

    /// Transaction status
    pub status: DBUserTransactionStatus,

    /// The timestamp when the transaction was created.
    pub created_at: DateTimeUtc,
    /// The timestamp when the transaction was completed.
    pub completed_at: Option<DateTimeUtc>,
    /// The timestamp when the transaction was cancelled.
    pub cancelled_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DBUserTransactionStatus {
    Pending,
    Completed,
    Cancelled,
}

impl From<UserTransactionStatus> for DBUserTransactionStatus {
    fn from(status: UserTransactionStatus) -> Self {
        match status {
            UserTransactionStatus::Pending => DBUserTransactionStatus::Pending,
            UserTransactionStatus::Completed => DBUserTransactionStatus::Completed,
            UserTransactionStatus::Cancelled => DBUserTransactionStatus::Cancelled,
        }
    }
}

impl From<DBUserTransactionStatus> for UserTransactionStatus {
    fn from(status: DBUserTransactionStatus) -> Self {
        match status {
            DBUserTransactionStatus::Pending => UserTransactionStatus::Pending,
            DBUserTransactionStatus::Completed => UserTransactionStatus::Completed,
            DBUserTransactionStatus::Cancelled => UserTransactionStatus::Cancelled,
        }
    }
}
